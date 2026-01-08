use mio::{Poll, Events, Interest, Token};
use mio::net::TcpListener;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::io;

use crate::core::connection::{Connection, State};

pub struct Server;

impl Server {
    pub fn start(addr: SocketAddr) {
        let mut poll = Poll::new().unwrap();
        let mut events = Events::with_capacity(128);

        let mut listener = TcpListener::bind(addr).unwrap();

        const SERVER: Token = Token(0);
        let mut next_token = 1;

        let mut connections: HashMap<Token, Connection> = HashMap::new();

        poll.registry()
            .register(&mut listener, SERVER, Interest::READABLE)
            .unwrap();

        loop {
            poll.poll(&mut events, None).unwrap();

            for event in events.iter() {
                match event.token() {
                    SERVER => {
                        loop {
                            match listener.accept() {
                                Ok((mut socket, _)) => {
                                    let token = Token(next_token);
                                    next_token += 1;

                                    poll.registry()
                                        .register(
                                            &mut socket,
                                            token,
                                            Interest::READABLE | Interest::WRITABLE,
                                        )
                                        .unwrap();

                                    connections.insert(token, Connection::new(socket));
                                }
                                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => break,
                                Err(e) => panic!("{}", e),
                            }
                        }
                    }
                    token => {
                        let close;

                        if let Some(conn) = connections.get_mut(&token) {
                            if event.is_readable() && conn.state == State::Reading {
                                let _ = conn.read();
                            }

                            if event.is_writable() && conn.state == State::Writing {
                                let _ = conn.write();
                            }

                            close = conn.state == State::Closed;
                        } else {
                            close = false;
                        }

                        if close {
                            connections.remove(&token);
                        }
                    }
                }
            }
        }
    }
}
