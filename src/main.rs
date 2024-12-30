use std::{fs, io, io::{BufReader, Write}, net::{TcpListener, TcpStream}, process};
use std::collections::HashMap;
use crate::http::{Method, Request, Response};

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

            let response = get_response(request);

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

fn get_response(request: Request) -> Response {
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "text/html".to_string());

    match request.method {
        Method::Get => {
            match get_file_contents(request.path.as_str()) {
                Ok(contents) => {
                    // TODO: content-type should be picked based on contents
                    Response::new("200 OK".to_string(), headers, contents)
                },
                Err(_) => {
                    // TODO: handle different errors differently
                    let body = "<html><body><h1>Error 500l: Internal Server Error</h1></body></html>".as_bytes();
                    Response::new("500 Internal Server Error".to_string(), headers, Vec::from(body))
                }
            }
        },
        _ => {
            let body = "<html><body><h1>Error 405: method not allowed</h1></body></html>".as_bytes();
            Response::new("405 Method Not Allowed".to_string(), headers, Vec::from(body))
        }
    }
}

fn get_file_contents(request_path: &str) -> Result<Vec<u8>, io::Error> {
    let file_path = get_file_path(request_path)?;
    fs::read(file_path)
}

const BASE_PATH: &str = "wwroot";

fn get_file_path(request_path: &str) -> Result<String, io::Error> {
    let project_dir = std::env::current_dir()?;
    let base = project_dir.join(BASE_PATH);
    let request_path = get_rerouted_path(request_path);
    
    println!("Base: {:?}", base);

    let requested = base.join(request_path.trim_start_matches("/"));

    if !requested.starts_with(base) {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "Path traversal not allowed"
        ));
    }

    println!("Requested path: {:?}", requested);

    if !requested.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "File not found"
        ));
    }

    Ok(requested.to_string_lossy().into_owned())
}

fn get_rerouted_path(path: &str) -> String {
    match path {
        "/" => "/index.html".to_string(),
        _ => path.to_string()
    }
}
