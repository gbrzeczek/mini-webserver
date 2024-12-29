use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;

#[derive(Debug)]
pub struct Response {
    status: String,
    headers: HashMap<String, String>,
    body: Vec<u8>
}

impl Response {
    pub fn new(status: String, headers: HashMap<String, String>, body: Vec<u8>) -> Response {
        Response {
            status,
            headers,
            body
        }
    }

    pub fn write_to(&self, stream: &mut TcpStream) -> std::io::Result<()> {
        stream.write_all(format!("HTTP/1.1 {}\r\n", self.status).as_bytes())?;

        for (key, value) in &self.headers {
            stream.write_all(format!("{}: {}\r\n", key, value).as_bytes())?;
        }

        stream.write_all(b"\r\n")?;

        if !self.body.is_empty() {
            stream.write_all(&self.body)?;
        }

        stream.flush()
    }
}