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
    pub fn ok(body: &str) -> Vec<u8> {
        format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        )
        .into_bytes()
    }
}
