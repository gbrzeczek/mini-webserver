use std::{
    io::{BufReader, Write},
    net::{TcpListener, TcpStream},
    process,
};
use std::collections::HashMap;
use crate::http::{Request, Response};

mod http;

fn main() {
    if let Err(message) = run() {
        eprintln!("{message}");
        process::exit(1);
    }
}

fn run() -> Result<(), String> {
    const URL: &str = "127.0.0.1:8080";

    let listener =
        TcpListener::bind(URL).map_err(|err| format!("Couldn't bind the address {URL}: {err}"))?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => respond_to_listener(stream),
            Err(message) => eprintln!("Failed to accept connection: {message}"),
        }
    }

    Ok(())
}

fn respond_to_listener(mut stream: TcpStream) {
    let mut reader = BufReader::new(&stream);

    match Request::parse(&mut reader) {
        Ok(request) => {
            println!("Request received: {:?}", request);

            let body = "<html><body><h1>Hello from Rust!</h1></body></html>".as_bytes();
            let mut headers = HashMap::new();
            headers.insert("Content-Type".to_string(), "text/html".to_string());
            let response = Response::new("200 OK".to_string(), headers, Vec::from(body));

            match response.write_to(&mut stream) {
                Ok(_) => match stream.flush() {
                    Ok(_) => println!("Response sent"),
                    Err(message) => eprintln!("Failed when flushing response buffer: {message}")
                }
                Err(message) => eprintln!("Failed to send response: {message}")
            }
        }
        Err(message) => eprintln!("Failed to read incoming stream: {}", message),
    }
}
