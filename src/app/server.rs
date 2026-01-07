use mio::{Poll, Events, Interest, Token};
use mio::net::{TcpListener, TcpStream};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::io::{self, Read};

pub struct Server;

impl Server {
    pub fn start() {
        let mut poll = Poll::new().expect("Poll failed");
        let mut events = Events::with_capacity(128);

        let addr: SocketAddr = "127.0.0.1:10000".parse().unwrap();
        let mut listener = TcpListener::bind(addr).expect("Bind failed");

        const SERVER: Token = Token(0);
        let mut next_token: usize = 1;
        let mut clients: HashMap<Token, TcpStream> = HashMap::new();

        poll.registry()
            .register(&mut listener, SERVER, Interest::READABLE)
            .expect("Register failed");

        println!("server booting!");

        loop {
            poll.poll(&mut events, None).expect("Poll error");

            for event in events.iter() {
                match event.token() {
                    SERVER => {
                        loop {
                            match listener.accept() {
                                Ok((mut socket, _)) => {
                                    let token = Token(next_token);
                                    next_token += 1;

                                    poll.registry()
                                        .register(&mut socket, token, Interest::READABLE)
                                        .expect("Register client failed");

                                    clients.insert(token, socket);
                                }
                                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => break,
                                Err(e) => panic!("accept error: {}", e),
                            }
                        }
                    }
                    token => {
                        let mut remove = false;

                        if let Some(socket) = clients.get_mut(&token) {
                            let mut buf = [0u8; 1024];
                            match socket.read(&mut buf) {
                                Ok(0) => remove = true,
                                Ok(n) => {
                                    let _ = &buf[..n];
                                }
                                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
                                Err(_) => remove = true,
                            }
                        }

                        if remove {
                            clients.remove(&token);
                        }
                    }
                }
            }
        }
    }
}
