pub struct Request {
    pub method: String,
    pub path: String,
    pub content_length: usize,
    pub body: Vec<u8>,
}

impl Request {
    pub fn parse(buf: &[u8]) -> Option<Self> {
        let text = std::str::from_utf8(buf).ok()?;
        let (head, body) = text.split_once("\r\n\r\n")?;

        let mut lines = head.lines();
        let first = lines.next()?;
        let mut parts = first.split_whitespace();

        let method = parts.next()?.to_string();
        let path = parts.next()?.to_string();

        let mut content_length = 0;
        for line in lines {
            if let Some(v) = line.strip_prefix("Content-Length:") {
                content_length = v.trim().parse().ok()?;
            }
        }

        if body.as_bytes().len() < content_length {
            return None;
        }

        Some(Self {
            method,
            path,
            content_length,
            body: body.as_bytes().to_vec(),
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

    pub fn bad_request() -> Vec<u8> {
        b"HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\n\r\n".to_vec()
    }
}
