use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub resource: String,
    pub protocol: HttpProtocol,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub params: Option<HashMap<String, String>>,
}

#[derive(Debug, Copy, Clone)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

#[derive(Debug, Copy, Clone)]
pub enum HttpProtocol {
    Http11,
}

pub fn parse_request(request: &str) -> Result<HttpRequest, String> {
    let mut lines = request.split("\r\n");
    let first_line = lines.next().ok_or("Invalid HTTP request")?;
    let mut first_line_parts = first_line.split_whitespace();
    let method = match first_line_parts.next().ok_or("Invalid HTTP method")? {
        "GET" => HttpMethod::Get,
        "POST" => HttpMethod::Post,
        "PUT" => HttpMethod::Put,
        "DELETE" => HttpMethod::Delete,
        _ => return Err("Unsupported HTTP method".to_string()),
    };
    let resource = first_line_parts
        .next()
        .ok_or("Invalid HTTP resource")?
        .to_string();
    let protocol = match first_line_parts.next().ok_or("Invalid HTTP protocol")? {
        "HTTP/1.1" => HttpProtocol::Http11,
        _ => return Err("Unsupported HTTP protocol".to_string()),
    };

    let mut headers = HashMap::new();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let mut header_parts = line.split(':');
        let header_name = header_parts
            .next()
            .ok_or("Invalid HTTP header")?
            .trim()
            .to_string();
        let header_value = header_parts
            .next()
            .ok_or("Invalid HTTP header value")?
            .trim()
            .to_string();
        headers.insert(header_name, header_value);
    }

    let body = lines.collect::<Vec<&str>>().join("\r\n");

    Ok(HttpRequest {
        method,
        resource,
        protocol,
        headers,
        body,
        params: None,
    })
}

impl fmt::Display for HttpRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}\r\n", self.method, self.resource, self.protocol)?;
        for (name, value) in &self.headers {
            write!(f, "{}: {}\r\n", name, value)?;
        }
        write!(f, "\r\n{}", self.body)
    }
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpMethod::Get => write!(f, "GET"),
            HttpMethod::Post => write!(f, "POST"),
            HttpMethod::Put => write!(f, "PUT"),
            HttpMethod::Delete => write!(f, "DELETE"),
        }
    }
}

impl fmt::Display for HttpProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpProtocol::Http11 => write!(f, "HTTP/1.1"),
        }
    }
}

impl AsRef<str> for HttpMethod {
    fn as_ref(&self) -> &str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
        }
    }
}
