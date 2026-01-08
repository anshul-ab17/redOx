use mio::net::TcpStream;
use std::io::{self, Read, Write};

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
}

impl Connection {
    pub fn new(socket: TcpStream) -> Self {
        Self {
            socket,
            buffer: Vec::new(),
            state: State::Reading,
        }
    }

    pub fn read(&mut self) -> io::Result<()> {
        let mut buf = [0u8; 1024];

        match self.socket.read(&mut buf) {
            Ok(0) => self.state = State::Closed,
            Ok(n) => {
                self.buffer.extend_from_slice(&buf[..n]);

                if let Some(req) = Request::parse(&self.buffer) {
                    let body = format!("Hello from Mio!\n{} {}", req.method, req.path);
                    self.buffer = Response::ok(&body);
                    self.state = State::Writing;
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
            Err(e) => return Err(e),
        }

        Ok(())
    }

    pub fn write(&mut self) -> io::Result<()> {
        if !self.buffer.is_empty() {
            self.socket.write(&self.buffer)?;
            self.buffer.clear();
        }

        self.state = State::Closed;
        Ok(())
    }
}
