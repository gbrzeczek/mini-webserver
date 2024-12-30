use std::{collections::HashMap, fs, io::{self, ErrorKind}};

use crate::{config::Config, http::{Method, Request, Response}};

pub fn get_response(request: Request) -> Response {
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "text/html".to_string());

    match request.method {
        Method::Get => {
            match get_file_contents(request.path.as_str()) {
                Ok(contents) => {
                    // TODO: content-type should be picked based on contents
                    Response::new("200 OK".to_string(), headers, contents)
                },
                Err(error) => {
                    match error.kind() {
                        ErrorKind::NotFound => {
                            let body = "<html><body><h1>Error 404: Not Found</h1></body></html>".as_bytes();
                            Response::new("404 Not Found".to_string(), headers, Vec::from(body))
                        },
                        ErrorKind::PermissionDenied => {
                            let body = "<html><body><h1>Error 403: Forbidden</h1></body></html>".as_bytes();
                            Response::new("403 Forbidden".to_string(), headers, Vec::from(body))
                        },
                        _ => {
                            let body = "<html><body><h1>Error 500: Internal Server Error</h1></body></html>".as_bytes();
                            Response::new("500 Internal Server Error".to_string(), headers, Vec::from(body))
                        }
                    }
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

fn get_file_path(request_path: &str) -> Result<String, io::Error> {
    let project_dir = std::env::current_dir()?;
    let base = project_dir.join(Config::base_path());
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
