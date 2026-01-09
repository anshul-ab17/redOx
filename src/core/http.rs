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
