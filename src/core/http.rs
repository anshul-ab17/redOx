use sha1::{Sha1, Digest};
use base64::{engine::general_purpose, Engine as _};

pub struct Request {
    pub method: String,
    pub path: String,
}

impl Request {
    pub fn parse(buf: &[u8]) -> Option<Self> {
        let text = std::str::from_utf8(buf).ok()?;
        let line = text.lines().next()?;
        let mut parts = line.split_whitespace();

        Some(Self {
            method: parts.next()?.to_string(),
            path: parts.next()?.to_string(),
        })
    }
}

pub struct Response;

impl Response {
    pub fn ok(body: &str) -> Vec<u8> {
        format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: keep-alive\r\n\r\n{}",
            body.len(),
            body
        )
        .into_bytes()
    }
}
pub fn is_websocket(buf: &[u8]) -> bool {
    let text = std::str::from_utf8(buf).unwrap_or("");
    text.contains("Upgrade: websocket")
}


pub fn ws_accept(key: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(format!("{}258EAFA5-E914-47DA-95CA-C5AB0DC85B11", key));
    let result = hasher.finalize();
    general_purpose::STANDARD.encode(result)
}
