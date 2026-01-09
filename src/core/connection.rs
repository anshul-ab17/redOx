use mio::net::TcpStream;
use std::io::{self, Read, Write};
use std::time::Instant;

use crate::core::http::{Request, Response};

#[derive(PartialEq)]
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
        loop {
            let mut buf = [0u8; 1024];

            match self.socket.read(&mut buf) {
                Ok(0) => {
                    self.state = State::Closed;
                    break;
                }
                Ok(n) => {
                    self.last_active = Instant::now();
                    self.buffer.extend_from_slice(&buf[..n]);

                    if let Some(req) = Request::parse(&self.buffer) {
                        let body = match req.path.as_str() {
                            "/" => "Edge Triggered Mio Server",
                            "/health" => "OK",
                            _ => "Not Found",
                        };

                        self.buffer = Response::ok(body);
                        self.state = State::Writing;
                        break;
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => break,
                Err(_) => {
                    self.state = State::Closed;
                    break;
                }
            }
        }

        Ok(())
    }

    pub fn write(&mut self) -> io::Result<()> {
        loop {
            if self.buffer.is_empty() {
                self.state = State::Reading;
                break;
            }

            match self.socket.write(&self.buffer) {
                Ok(n) => {
                    self.last_active = Instant::now();
                    self.buffer.drain(..n);
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => break,
                Err(_) => {
                    self.state = State::Closed;
                    break;
                }
            }
        }

        Ok(())
    }

    pub fn is_timed_out(&self, secs: u64) -> bool {
        self.last_active.elapsed().as_secs() > secs
    }
}
