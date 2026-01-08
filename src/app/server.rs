use mio::{Poll, Events, Interest, Token};
use mio::net::{TcpListener, TcpStream};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::io::{self, Read, Write};

pub struct Server;

impl Server {
    pub fn start(addr: SocketAddr) {
        let mut poll = Poll::new().unwrap();
        let mut events = Events::with_capacity(128);

        let mut listener = TcpListener::bind(addr).unwrap();

        const SERVER: Token = Token(0);
        let mut next_token = 1;

        let mut clients = HashMap::new();
        let mut buffers = HashMap::new();

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

                                    clients.insert(token, socket);
                                    buffers.insert(token, Vec::new());
                                }
                                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => break,
                                Err(e) => panic!("{}", e),
                            }
                        }
                    }
                    token => {
                        let mut close = false;

                        if let Some(socket) = clients.get_mut(&token) {
                            if event.is_readable() {
                                let mut buf = [0u8; 1024];
                                match socket.read(&mut buf) {
                                    Ok(0) => close = true,
                                    Ok(n) => buffers.get_mut(&token).unwrap().extend_from_slice(&buf[..n]),
                                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
                                    Err(_) => close = true,
                                }
                            }

                            if event.is_writable() {
                                if let Some(data) = buffers.get_mut(&token) {
                                    if !data.is_empty() {
                                        let _ = socket.write(data);
                                        data.clear();
                                    }
                                }
                            }
                        }

                        if close {
                            clients.remove(&token);
                            buffers.remove(&token);
                        }
                    }
                }
            }
        }
    }
}
