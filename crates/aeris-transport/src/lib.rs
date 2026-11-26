#![forbid(unsafe_code)]

//! Explicit-address socket primitives. Routing and DNS policy are intentionally
//! outside this crate. No socket is opened by constructing a Transport.

use std::fmt;
use std::future::Future;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, UdpSocket};
use tokio::sync::{OwnedSemaphorePermit, Semaphore, watch};

pub const MAX_UDP_PAYLOAD: usize = 65_507;
pub const UDP_RECEIVE_BUFFER: usize = 65_535;

#[derive(Clone, Copy, Debug)]
pub struct Settings {
    pub max_sockets: usize,
    pub capacity_timeout: Duration,
    pub connect_timeout: Duration,
    pub io_timeout: Duration,
    pub tcp_nodelay: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            max_sockets: 256,
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
            Arc::clone(&self.capacity)
                .acquire_owned()
                .await
                .map_err(|_| Error::Cancelled)
        })
        .await
    }
}

/// A shared transport lifecycle. The session limit counts open TCP streams,
/// bound UDP sockets and in-progress connection/bind attempts.
#[derive(Clone)]
pub struct Transport {
    control: Arc<Control>,
}

impl Transport {
    pub fn new(settings: Settings) -> Result<Self, Error> {
        let valid_timeout = |d: Duration| !d.is_zero() && d <= Duration::from_secs(86_400);
        if !(1..=65_536).contains(&settings.max_sockets)
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
                stop,
            }),
        })
    }

    /// Stops new and pending operations on all clones. Owners must drop their
    /// idle socket handles to release OS resources. This is not a Kill Switch.
    pub fn shutdown(&self) {
        self.control.stop.send_replace(true);
        self.control.capacity.close();
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
        self.control
            .io(
                Operation::Read,
                self.control.settings.io_timeout,
                self.stream.read(buffer),
            )
            .await
    }

    /// One partial write; caller advances by the returned byte count.
    pub async fn write(&mut self, buffer: &[u8]) -> Result<usize, Error> {
        self.control
            .io(
                Operation::Write,
                self.control.settings.io_timeout,
                self.stream.write(buffer),
            )
            .await
    }

    pub async fn shutdown_write(&mut self) -> Result<(), Error> {
        self.control
            .io(
                Operation::HalfClose,
                self.control.settings.io_timeout,
                self.stream.shutdown(),
            )
            .await
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
