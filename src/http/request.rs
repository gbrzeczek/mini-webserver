use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    net::TcpStream,
};

use super::method::Method;

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: std::collections::HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Request {
    pub fn parse(reader: &mut BufReader<&TcpStream>) -> Result<Request, String> {
        let mut lines = reader.lines();

        let line = lines
            .next()
            .ok_or("The request was empty")?
            .map_err(|err| format!("Error occurred while reading parts: {err}"))?;

        let parts: Vec<_> = line.split_whitespace().collect();

        if parts.len() < 2 {
            return Err("Invalid request line".to_string());
        }

        let method = Method::parse(parts[0])?;
        let path = parts[1].to_string();

        let mut headers: HashMap<String, String> = HashMap::new();

        loop {
            let line = lines
                .next()
                .ok_or("The request was empty")?
                .map_err(|err| format!("Error occurred while reading parts: {err}"))?;
           
            let line = line.trim();
            
            if line.is_empty() {
                break;
            }
            
            if let Some((key, value)) = line.split_once(":") {
                headers.insert(key.trim().to_lowercase(), value.trim().to_string());
            }
        }

        Ok(Request {
            method,
            path,
            headers,
            body: Vec::new()
        })
    }
}
