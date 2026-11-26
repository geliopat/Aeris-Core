use aeris_transport::{Error, MAX_UDP_PAYLOAD, Operation, Settings, Transport, UDP_RECEIVE_BUFFER};
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

fn loopback() -> SocketAddr {
    SocketAddr::from((Ipv4Addr::LOCALHOST, 0))
}

fn loopback_v6() -> SocketAddr {
    SocketAddr::from((Ipv6Addr::LOCALHOST, 0))
}

fn settings() -> Settings {
    Settings {
        max_sockets: 2,
        capacity_timeout: Duration::from_millis(100),
        connect_timeout: Duration::from_secs(3),
        io_timeout: Duration::from_secs(3),
        ..Settings::default()
    }
}

#[test]
fn invalid_limits_are_rejected() {
    assert!(matches!(
        Transport::new(Settings {
            max_sockets: 0,
            ..settings()
        }),
        Err(Error::InvalidSettings)
    ));
    assert!(matches!(
        Transport::new(Settings {
            io_timeout: Duration::ZERO,
            ..settings()
        }),
        Err(Error::InvalidSettings)
    ));
    assert!(matches!(
        Transport::new(Settings {
            max_pending_sockets: 65_537,
            ..settings()
        }),
        Err(Error::InvalidSettings)
    ));
}

#[tokio::test]
async fn tcp_round_trip_and_half_close() {
    tcp_round_trip(loopback()).await;
}

#[tokio::test]
async fn tcp_round_trip_and_half_close_ipv6() {
    tcp_round_trip(loopback_v6()).await;
}

async fn tcp_round_trip(bind_address: SocketAddr) {
    let listener = TcpListener::bind(bind_address).await.expect("bind");
    let address = listener.local_addr().expect("address");
    let server = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.expect("accept");
        let mut request = [0; 4];
        stream.read_exact(&mut request).await.expect("request");
        stream.write_all(&request).await.expect("echo");
        let mut last = [0; 1];
        assert_eq!(stream.read(&mut last).await.expect("EOF"), 0);
        stream.shutdown().await.expect("half close");
    });
    let transport = Transport::new(settings()).expect("transport");
    let mut stream = transport.connect_tcp(address).await.expect("connect");
    assert_eq!(stream.peer_addr().expect("peer"), address);
    let payload = b"ping";
    let mut sent = 0;
    while sent < payload.len() {
        let n = stream.write(&payload[sent..]).await.expect("write");
        assert!(n > 0);
        sent += n;
    }
    stream.shutdown_write().await.expect("half close");
    let mut reply = [0; 4];
    let mut received = 0;
    while received < reply.len() {
        let n = stream.read(&mut reply[received..]).await.expect("read");
        assert!(n > 0);
        received += n;
    }
    assert_eq!(&reply, payload);
    assert_eq!(stream.read(&mut [0; 1]).await.expect("EOF"), 0);
    tokio::time::timeout(Duration::from_secs(5), server)
        .await
        .expect("server deadline")
        .expect("server");
}

#[tokio::test]
async fn tcp_split_can_read_while_writing_and_half_closing() {
    let listener = TcpListener::bind(loopback()).await.expect("bind");
    let address = listener.local_addr().expect("address");
    let server = async {
        let (mut stream, _) = listener.accept().await.expect("accept");
        let mut request = Vec::new();
        stream
            .read_to_end(&mut request)
            .await
            .expect("request and EOF");
        assert_eq!(request, b"?\n");
        stream.write_all(b"!\n").await.expect("reply");
    };
    let client = async {
        let transport = Transport::new(settings()).expect("transport");
        let mut connection = transport.connect_tcp(address).await.expect("connect");
        let (mut reader, mut writer) = connection.split();
        let receive = async {
            let mut reply = [0; 2];
            let mut received = 0;
            while received < reply.len() {
                let count = reader.read(&mut reply[received..]).await.expect("read");
                assert!(count > 0);
                received += count;
            }
            assert_eq!(&reply, b"!\n");
        };
        let send = async {
            let payload = b"?\n";
            let mut sent = 0;
            while sent < payload.len() {
                let count = writer.write(&payload[sent..]).await.expect("write");
                assert!(count > 0);
                sent += count;
            }
            writer.shutdown_write().await.expect("half close");
        };
        tokio::join!(biased; receive, send);
    };
    tokio::time::timeout(Duration::from_secs(5), async {
        tokio::join!(server, client);
    })
    .await
    .expect("round trip deadline");
}

#[tokio::test]
async fn udp_round_trip_and_buffer_limits() {
    udp_round_trip(loopback()).await;
}

#[tokio::test]
async fn udp_round_trip_and_buffer_limits_ipv6() {
    udp_round_trip(loopback_v6()).await;
}

async fn udp_round_trip(bind_address: SocketAddr) {
    let transport = Transport::new(settings()).expect("transport");
    let sender = transport.bind_udp(bind_address).await.expect("sender");
    let receiver = transport.bind_udp(bind_address).await.expect("receiver");
    let address = receiver.local_addr().expect("address");
    assert_eq!(sender.send_to(b"hello", address).await.expect("send"), 5);
    let mut buffer = vec![0; UDP_RECEIVE_BUFFER];
    let (size, from) = receiver.recv_from(&mut buffer).await.expect("receive");
    assert_eq!(&buffer[..size], b"hello");
    assert_eq!(from, sender.local_addr().expect("sender address"));

    assert_eq!(
        sender.send_to(&vec![0; MAX_UDP_PAYLOAD + 1], address).await,
        Err(Error::DatagramTooLarge)
    );
    assert_eq!(
        receiver.recv_from(&mut [0; 16]).await,
        Err(Error::ReceiveBufferTooSmall)
    );
}

#[tokio::test]
async fn socket_capacity_is_shared_and_released() {
    let transport = Transport::new(Settings {
        max_sockets: 1,
        ..settings()
    })
    .expect("transport");
    let first = transport.bind_udp(loopback()).await.expect("first");
    let other = transport.clone();
    assert!(matches!(
        other.bind_udp(loopback()).await,
        Err(Error::Timeout(Operation::Capacity))
    ));
    drop(first);
    assert!(other.bind_udp(loopback()).await.is_ok());
}

#[tokio::test]
async fn waiting_capacity_is_bounded_and_dropped_waiters_release_it() {
    let transport = Transport::new(Settings {
        max_sockets: 1,
        max_pending_sockets: 1,
        capacity_timeout: Duration::from_secs(3),
        ..settings()
    })
    .expect("transport");
    let first = transport.bind_udp(loopback()).await.expect("first");
    let other = transport.clone();
    let mut pending = Box::pin(transport.bind_udp(loopback()));
    tokio::select! {
        biased;
        _ = &mut pending => panic!("socket slot should be occupied"),
        () = std::future::ready(()) => {}
    }
    assert!(matches!(
        other.connect_tcp(loopback()).await,
        Err(Error::CapacityQueueFull)
    ));
    drop(pending);

    let replacement = other.bind_udp(loopback());
    tokio::pin!(replacement);
    tokio::select! {
        biased;
        _ = &mut replacement => panic!("replacement should obtain the freed queue slot and wait"),
        () = std::future::ready(()) => {}
    }
    drop(first);
    assert!(replacement.await.is_ok());
}

#[tokio::test]
async fn zero_pending_capacity_rejects_overload_without_rejecting_available_slots() {
    let transport = Transport::new(Settings {
        max_sockets: 1,
        max_pending_sockets: 0,
        ..settings()
    })
    .expect("transport");
    let first = transport.bind_udp(loopback()).await.expect("first");
    assert!(matches!(
        transport.bind_udp(loopback()).await,
        Err(Error::CapacityQueueFull)
    ));
    drop(first);
    assert!(transport.bind_udp(loopback()).await.is_ok());
}

#[tokio::test]
async fn idle_udp_receive_times_out() {
    let transport = Transport::new(Settings {
        io_timeout: Duration::from_millis(100),
        ..settings()
    })
    .expect("transport");
    let receiver = transport.bind_udp(loopback()).await.expect("receiver");
    let mut buffer = vec![0; UDP_RECEIVE_BUFFER];
    assert_eq!(
        receiver.recv_from(&mut buffer).await,
        Err(Error::Timeout(Operation::Receive))
    );
}

#[tokio::test]
async fn stop_interrupts_receive_and_waiting_capacity() {
    let transport = Transport::new(Settings {
        max_sockets: 1,
        capacity_timeout: Duration::from_secs(30),
        io_timeout: Duration::from_secs(30),
        ..settings()
    })
    .expect("transport");
    let receiver = transport.bind_udp(loopback()).await.expect("receiver");
    let waiting = transport.clone();
    let read_task = tokio::spawn(async move {
        let mut buffer = vec![0; UDP_RECEIVE_BUFFER];
        receiver.recv_from(&mut buffer).await
    });
    let capacity_task =
        tokio::spawn(
            async move { matches!(waiting.bind_udp(loopback()).await, Err(Error::Cancelled)) },
        );
    tokio::task::yield_now().await;
    transport.shutdown();
    assert_eq!(
        tokio::time::timeout(Duration::from_secs(3), read_task)
            .await
            .expect("cancel deadline")
            .expect("read task"),
        Err(Error::Cancelled)
    );
    assert!(
        tokio::time::timeout(Duration::from_secs(3), capacity_task)
            .await
            .expect("capacity deadline")
            .expect("capacity task")
    );
    assert!(matches!(
        transport.bind_udp(loopback()).await,
        Err(Error::Cancelled)
    ));
}
