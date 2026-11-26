#![forbid(unsafe_code)]

//! Explicit-address socket primitives. Routing and DNS policy are intentionally
//! outside this crate. No socket is opened by constructing a Transport.

use std::fmt;
use std::future::Future;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::tcp::{ReadHalf, WriteHalf};
use tokio::net::{TcpStream, UdpSocket};
use tokio::sync::{OwnedSemaphorePermit, Semaphore, TryAcquireError, watch};

pub const MAX_UDP_PAYLOAD: usize = 65_507;
pub const UDP_RECEIVE_BUFFER: usize = 65_535;

#[derive(Clone, Copy, Debug)]
pub struct Settings {
    pub max_sockets: usize,
    /// Maximum number of connect or bind requests waiting for a socket slot.
    /// Zero requires requests to obtain a slot immediately.
    pub max_pending_sockets: usize,
    pub capacity_timeout: Duration,
    pub connect_timeout: Duration,
    pub io_timeout: Duration,
    pub tcp_nodelay: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            max_sockets: 256,
            max_pending_sockets: 256,
            capacity_timeout: Duration::from_secs(5),
            connect_timeout: Duration::from_secs(10),
            io_timeout: Duration::from_secs(30),
            tcp_nodelay: true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operation {
    Capacity,
    Connect,
    Bind,
    Read,
    Write,
    HalfClose,
    Receive,
    Send,
    Address,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidSettings,
    Cancelled,
    CapacityQueueFull,
    Timeout(Operation),
    Io(Operation, io::ErrorKind),
    DatagramTooLarge,
    ReceiveBufferTooSmall,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSettings => f.write_str("invalid transport settings"),
            Self::Cancelled => f.write_str("transport stopped"),
            Self::CapacityQueueFull => f.write_str("transport socket wait queue is full"),
            Self::Timeout(op) => write!(f, "transport {op:?} timed out"),
            Self::Io(op, kind) => write!(f, "transport {op:?} failed: {kind:?}"),
            Self::DatagramTooLarge => f.write_str("datagram exceeds transport payload limit"),
            Self::ReceiveBufferTooSmall => f.write_str("UDP receive buffer too small"),
        }
    }
}

impl std::error::Error for Error {}

struct Control {
    settings: Settings,
    capacity: Arc<Semaphore>,
    pending: Semaphore,
    stop: watch::Sender<bool>,
}

async fn cancelled(mut stop: watch::Receiver<bool>) {
    loop {
        if *stop.borrow_and_update() {
            return;
        }
        if stop.changed().await.is_err() {
            return;
        }
    }
}

impl Control {
    async fn run<T>(
        &self,
        operation: Operation,
        timeout: Duration,
        future: impl Future<Output = Result<T, Error>>,
    ) -> Result<T, Error> {
        tokio::select! {
            biased;
            () = cancelled(self.stop.subscribe()) => Err(Error::Cancelled),
            result = tokio::time::timeout(timeout, future) => {
                result.map_err(|_| Error::Timeout(operation))?
            }
        }
    }

    async fn io<T>(
        &self,
        operation: Operation,
        timeout: Duration,
        future: impl Future<Output = io::Result<T>>,
    ) -> Result<T, Error> {
        self.run(operation, timeout, async {
            future.await.map_err(|err| Error::Io(operation, err.kind()))
        })
        .await
    }

    async fn reserve(&self) -> Result<OwnedSemaphorePermit, Error> {
        self.run(Operation::Capacity, self.settings.capacity_timeout, async {
            match Arc::clone(&self.capacity).try_acquire_owned() {
                Ok(permit) => return Ok(permit),
                Err(TryAcquireError::Closed) => return Err(Error::Cancelled),
                Err(TryAcquireError::NoPermits) => {}
            }
            let _waiting = self.pending.try_acquire().map_err(|err| match err {
                TryAcquireError::Closed => Error::Cancelled,
                TryAcquireError::NoPermits => Error::CapacityQueueFull,
            })?;
            Arc::clone(&self.capacity)
                .acquire_owned()
                .await
                .map_err(|_| Error::Cancelled)
        })
        .await
    }

    async fn read_tcp(
        &self,
        stream: &mut (impl AsyncRead + Unpin),
        buffer: &mut [u8],
    ) -> Result<usize, Error> {
        self.io(
            Operation::Read,
            self.settings.io_timeout,
            stream.read(buffer),
        )
        .await
    }

    async fn write_tcp(
        &self,
        stream: &mut (impl AsyncWrite + Unpin),
        buffer: &[u8],
    ) -> Result<usize, Error> {
        self.io(
            Operation::Write,
            self.settings.io_timeout,
            stream.write(buffer),
        )
        .await
    }

    async fn shutdown_tcp(&self, stream: &mut (impl AsyncWrite + Unpin)) -> Result<(), Error> {
        self.io(
            Operation::HalfClose,
            self.settings.io_timeout,
            stream.shutdown(),
        )
        .await
    }
}

/// A shared transport lifecycle. The session limit counts open TCP streams,
/// bound UDP sockets and in-progress connection/bind attempts. Waiting
/// attempts share a separate bounded queue across all clones.
#[derive(Clone)]
pub struct Transport {
    control: Arc<Control>,
}

impl Transport {
    pub fn new(settings: Settings) -> Result<Self, Error> {
        let valid_timeout = |d: Duration| !d.is_zero() && d <= Duration::from_secs(86_400);
        if !(1..=65_536).contains(&settings.max_sockets)
            || settings.max_pending_sockets > 65_536
            || !valid_timeout(settings.capacity_timeout)
            || !valid_timeout(settings.connect_timeout)
            || !valid_timeout(settings.io_timeout)
        {
            return Err(Error::InvalidSettings);
        }
        let (stop, _) = watch::channel(false);
        Ok(Self {
            control: Arc::new(Control {
                settings,
                capacity: Arc::new(Semaphore::new(settings.max_sockets)),
                pending: Semaphore::new(settings.max_pending_sockets),
                stop,
            }),
        })
    }

    /// Stops new and pending operations on all clones. Owners must drop their
    /// idle socket handles to release OS resources. This is not a Kill Switch.
    pub fn shutdown(&self) {
        self.control.stop.send_replace(true);
        self.control.capacity.close();
        self.control.pending.close();
    }

    pub async fn connect_tcp(&self, address: SocketAddr) -> Result<TcpConnection, Error> {
        let permit = self.control.reserve().await?;
        let stream = self
            .control
            .io(
                Operation::Connect,
                self.control.settings.connect_timeout,
                TcpStream::connect(address),
            )
            .await?;
        stream
            .set_nodelay(self.control.settings.tcp_nodelay)
            .map_err(|err| Error::Io(Operation::Connect, err.kind()))?;
        Ok(TcpConnection {
            stream,
            control: Arc::clone(&self.control),
            _permit: permit,
        })
    }

    pub async fn bind_udp(&self, address: SocketAddr) -> Result<UdpConnection, Error> {
        let permit = self.control.reserve().await?;
        let socket = self
            .control
            .io(
                Operation::Bind,
                self.control.settings.connect_timeout,
                UdpSocket::bind(address),
            )
            .await?;
        Ok(UdpConnection {
            socket,
            control: Arc::clone(&self.control),
            _permit: permit,
        })
    }
}

pub struct TcpConnection {
    stream: TcpStream,
    control: Arc<Control>,
    _permit: OwnedSemaphorePermit,
}

impl TcpConnection {
    pub fn local_addr(&self) -> Result<SocketAddr, Error> {
        self.stream
            .local_addr()
            .map_err(|err| Error::Io(Operation::Address, err.kind()))
    }

    pub fn peer_addr(&self) -> Result<SocketAddr, Error> {
        self.stream
            .peer_addr()
            .map_err(|err| Error::Io(Operation::Address, err.kind()))
    }

    /// May return fewer bytes than requested. A nonempty buffer and zero bytes
    /// read means EOF. No automatic replay, reconnection or fallback occurs.
    pub async fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        self.control.read_tcp(&mut self.stream, buffer).await
    }

    /// One partial write; caller advances by the returned byte count.
    pub async fn write(&mut self, buffer: &[u8]) -> Result<usize, Error> {
        self.control.write_tcp(&mut self.stream, buffer).await
    }

    pub async fn shutdown_write(&mut self) -> Result<(), Error> {
        self.control.shutdown_tcp(&mut self.stream).await
    }

    /// Borrow independent read and write halves for full-duplex I/O. Both
    /// halves use the connection's existing capacity slot and stop signal.
    pub fn split(&mut self) -> (TcpReader<'_>, TcpWriter<'_>) {
        let (reader, writer) = self.stream.split();
        (
            TcpReader {
                reader,
                control: &self.control,
            },
            TcpWriter {
                writer,
                control: &self.control,
            },
        )
    }
}

/// Read side of a borrowed TCP connection.
pub struct TcpReader<'a> {
    reader: ReadHalf<'a>,
    control: &'a Control,
}

impl TcpReader<'_> {
    /// Reads up to the buffer length. Zero with a nonempty buffer means EOF.
    pub async fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        self.control.read_tcp(&mut self.reader, buffer).await
    }
}

/// Write side of a borrowed TCP connection.
pub struct TcpWriter<'a> {
    writer: WriteHalf<'a>,
    control: &'a Control,
}

impl TcpWriter<'_> {
    /// One partial write; caller advances by the returned byte count.
    pub async fn write(&mut self, buffer: &[u8]) -> Result<usize, Error> {
        self.control.write_tcp(&mut self.writer, buffer).await
    }

    /// Closes the write side while leaving the read side available.
    pub async fn shutdown_write(&mut self) -> Result<(), Error> {
        self.control.shutdown_tcp(&mut self.writer).await
    }
}

pub struct UdpConnection {
    socket: UdpSocket,
    control: Arc<Control>,
    _permit: OwnedSemaphorePermit,
}

impl UdpConnection {
    pub fn local_addr(&self) -> Result<SocketAddr, Error> {
        self.socket
            .local_addr()
            .map_err(|err| Error::Io(Operation::Address, err.kind()))
    }

    pub async fn send_to(&self, buffer: &[u8], address: SocketAddr) -> Result<usize, Error> {
        if buffer.len() > MAX_UDP_PAYLOAD {
            return Err(Error::DatagramTooLarge);
        }
        self.control
            .io(
                Operation::Send,
                self.control.settings.io_timeout,
                self.socket.send_to(buffer, address),
            )
            .await
    }

    /// Caller-provided full-size buffer avoids silent truncation of normal UDP
    /// datagrams. IPv6 jumbograms are not supported by this initial interface.
    pub async fn recv_from(&self, buffer: &mut [u8]) -> Result<(usize, SocketAddr), Error> {
        if buffer.len() < UDP_RECEIVE_BUFFER {
            return Err(Error::ReceiveBufferTooSmall);
        }
        self.control
            .io(
                Operation::Receive,
                self.control.settings.io_timeout,
                self.socket.recv_from(buffer),
            )
            .await
    }
}
