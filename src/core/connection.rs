use mio::net::TcpStream;
use std::io::{self, Read, Write};
use std::time::Instant;

use crate::core::http::{Request, Response};

#[derive(Debug, PartialEq)]
pub enum State {
    Reading,
    Writing,
    Closed,
}

pub struct Connection {
    pub socket: TcpStream,
    pub buffer: Vec<u8>,
    pub state: State,
    pub last_active: Instant,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Self {
        Self {
            socket,
            buffer: Vec::new(),
            state: State::Reading,
            last_active: Instant::now(),
        }
    }

    pub fn read(&mut self) -> io::Result<()> {
        self.last_active = Instant::now();

        let mut buf = [0u8; 1024];

        match self.socket.read(&mut buf) {
            Ok(0) => self.state = State::Closed,
            Ok(n) => {
                self.buffer.extend_from_slice(&buf[..n]);

                if let Some(req) = Request::parse(&self.buffer) {
                    let response = match (req.method.as_str(), req.path.as_str()) {
                        ("GET", "/") => Response::ok("Mio HTTP Server"),
                        _ => Response::bad_request(),
                    };

                    self.buffer = response;
                    self.state = State::Writing;
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
            Err(_) => self.state = State::Closed,
        }

        Ok(())
    }

    pub fn write(&mut self) -> io::Result<()> {
        self.last_active = Instant::now();

        if !self.buffer.is_empty() {
            self.socket.write(&self.buffer)?;
            self.buffer.clear();
        }

        self.state = State::Reading;
        Ok(())
    }

    pub fn is_timed_out(&self, timeout_secs: u64) -> bool {
        self.last_active.elapsed().as_secs() > timeout_secs
    }
}
