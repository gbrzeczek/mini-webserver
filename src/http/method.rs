#[derive(Debug)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Options
}

impl Method {
    pub fn parse(s: &str) -> Result<Method, String> {
        let e = match s.to_uppercase().as_str() {
            "GET" => Method::Get,
            "POST" => Method::Post,
            "PUT" => Method::Put,
            "DELETE" => Method::Delete,
            "PATCH" => Method::Patch,
            "OPTIONS" => Method::Options,
            _ => return Err("Invalid request method".to_string())
        };

        Ok(e)
    }
}
