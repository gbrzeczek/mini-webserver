use crate::file_reader::{CachedFileReader, FileReader};
use crate::http::{Method, Request, Response};
use std::{collections::HashMap, io::ErrorKind};

pub fn get_response(request: Request) -> Response {
    let request_path = get_rerouted_path(request.path.as_str());
    let reader = CachedFileReader::instance();

    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "text/html".to_string());

    match request.method {
        Method::Get => match reader.read_file(request_path.as_str()) {
            Ok(contents) => Response::new("200 OK".to_string(), headers, contents),
            Err(error) => match error.kind() {
                ErrorKind::NotFound => {
                    let body = "<html><body><h1>Error 404: Not Found</h1></body></html>".as_bytes();
                    Response::new("404 Not Found".to_string(), headers, Vec::from(body))
                }
                _ => {
                    let body =
                        "<html><body><h1>Error 500: Internal Server Error</h1></body></html>"
                            .as_bytes();
                    Response::new(
                        "500 Internal Server Error".to_string(),
                        headers,
                        Vec::from(body),
                    )
                }
            },
        },
        _ => {
            let body =
                "<html><body><h1>Error 405: method not allowed</h1></body></html>".as_bytes();
            Response::new(
                "405 Method Not Allowed".to_string(),
                headers,
                Vec::from(body),
            )
        }
    }
}

fn get_rerouted_path(path: &str) -> String {
    match path {
        "/" => "/index.html".to_string(),
        _ => path.to_string(),
    }
}
