pub fn json(body: &str) -> Vec<u8> {
    let content = format!("{{{}}}", body);
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        content.len(),
        content
    )
    .into_bytes()
}
