use mio::{Poll, Events, Interest, Token};
use mio::net::TcpListener;
use std::net::SocketAddr;
use std::io;

pub struct Server;

impl Server {
    pub fn start() {
        let mut poll = Poll::new().expect("Poll failed");
        let mut events = Events::with_capacity(128);

        let addr: SocketAddr = "127.0.0.1:10000".parse().unwrap();
        let mut listener = TcpListener::bind(addr).expect("Bind failed");

        const SERVER: Token = Token(0);

        poll.registry()
            .register(&mut listener, SERVER, Interest::READABLE)
            .expect("Register failed");

        println!("server booting!");

        loop {
            poll.poll(&mut events, None).expect("Poll error");

            for event in events.iter() {
                if event.token() == SERVER && event.is_readable() {
                    loop {
                        match listener.accept() {
                            Ok((_socket, addr)) => {
                                println!("client connected: {}", addr);
                            }
                            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                                break;
                            }
                            Err(e) => panic!("accept error: {}", e),
                        }
                    }
                }
            }
        }
    }
}
