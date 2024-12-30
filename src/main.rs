use std::{io::{BufReader, Write}, net::{TcpListener, TcpStream}, process};

use crate::http::Request;

mod http;
mod router;
mod config;

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

            let response = router::get_response(request);

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

