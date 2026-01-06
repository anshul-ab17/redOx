use mio::{Events, Interest, Poll, Token};
use mio::net::TcpListener;
use std::io;
use std::net::SocketAddr;

const SERVER: Token = Token(0);

pub struct Server;

impl Server {
    pub fn start() -> io::Result<()> {
        let mut poll = Poll::new()?;
        let mut events = Events::with_capacity(128);

        let addr: SocketAddr = "127.0.0.1:9000".parse().unwrap();
        let mut listener = TcpListener::bind(addr)?;

        poll.registry().register(
            &mut listener,
            SERVER,
            Interest::READABLE,
        )?;

        loop {
            poll.poll(&mut events, None)?;

            for event in events.iter() {
                match event.token() {
                    SERVER => {
                        loop {
                            match listener.accept() {
                                Ok((_socket, _addr)) => {}
                                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => break,
                                Err(e) => return Err(e),
                            }
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}