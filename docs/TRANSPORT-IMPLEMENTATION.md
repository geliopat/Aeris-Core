# Transport implementation status / Статус сетевого транспорта

Initial isolated transport crate, stacked on the configuration work; not a running VPN.

- Tokio 1.54.1 (MIT metadata at the verified upstream tag), with explicit minimal features and Cargo.lock generated in the first Windows CI and checked in.
- TCP connect, partial read/write, address metadata and write half-close. Borrowed TCP halves allow reading and writing concurrently while sharing the same socket slot and stop signal. UDP bind/send/receive uses caller-provided buffers. Numeric `SocketAddr` values leave DNS and path selection to the caller.
- Shared lifetime capacity via semaphore, including connect/bind attempts; RAII release on handle drop. Separate capacity, connect and I/O deadlines; shutdown wakes pending operations and prevents new I/O. Owners must still drop idle handles. This is not WFP/Kill Switch enforcement.
- A separate queue bounds the connect and bind operations waiting for a socket slot across all clones. A full queue returns `CapacityQueueFull`. A wait can also time out or be cancelled and dropping a waiting future returns its slot. Callers remain responsible for bounding their own job queues. UDP uses a conservative portable send size and requires a full-size receive buffer. IPv6 jumbograms remain outside this interface.
- Provisional internal settings cover 256 live sockets, 256 waiters, a 5 s capacity deadline, a 10 s connect deadline, a 30 s I/O deadline and TCP_NODELAY enabled. Zero waiters enables immediate rejection when the limit is reached. Benchmark before treating any limit as a product default.
- Loopback coverage includes IPv4 and IPv6 TCP echo and half-close, full-duplex TCP, IPv4 and IPv6 UDP datagrams, buffer limits, queue saturation, cancelled waiter release, read deadlines and shutdown cancellation. Route binding, real-world congestion, drivers, TLS and protocol interoperation remain to be tested in later stages.
- Do not wire direct transport to GUI/CLI or turn it into a network service until routing, DNS policy and lifecycle contracts are implemented.
- First Windows CI (45839430097): all 22 workspace tests, Clippy and build passed after temporary lockfile generation and formatting. Both temporary steps have now been removed; the committed lockfile and formatting require a new strict CI run. Dependency license/advisory review is required before merge.

Source: https://github.com/tokio-rs/tokio/tree/tokio-1.54.1
