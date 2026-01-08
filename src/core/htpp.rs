pub struct Request {
    pub method: String,
    pub path: String,
}

impl Request {
    pub fn parse(buf: &[u8]) -> Option<Self> {
        let text = std::str::from_utf8(buf).ok()?;
        let mut lines = text.lines();
        let first = lines.next()?;

        let mut parts = first.split_whitespace();
        let method = parts.next()?.to_string();
        let path = parts.next()?.to_string();

        Some(Self { method, path })
    }
}

pub struct Response;

impl Response {
    pub fn ok(body: &str, keep_alive: bool) -> Vec<u8> {
        let conn = if keep_alive { "keep-alive" } else { "close" };

        format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: {}\r\n\r\n{}",
            body.len(),
            conn,
            body
        )
        .into_bytes()
    }

    pub fn not_found() -> Vec<u8> {
        b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n".to_vec()
    }
}
