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

impl From<&str> for HttpMethod {
    fn from(s: &str) -> Self {
        match s {
            "GET" => HttpMethod::Get,
            "POST" => HttpMethod::Post,
            "PUT" => HttpMethod::Put,
            "DELETE" => HttpMethod::Delete,
            _ => panic!("Unsupported HTTP method"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum HttpProtocol {
    Http11,
}

impl From<&str> for HttpProtocol {
    fn from(s: &str) -> Self {
        match s {
            "HTTP/1.1" => HttpProtocol::Http11,
            _ => panic!("Unsupported HTTP protocol"),
        }
    }
}

impl From<&str> for HttpRequest {
    fn from(request: &str) -> Self {
        let mut lines = request.split("\r\n");
        let first_line = lines.next().expect("Invalid HTTP request");
        let mut first_line_parts = first_line.split_whitespace();
        let method = HttpMethod::from(first_line_parts.next().expect("Invalid HTTP method"));
        let resource = first_line_parts
            .next()
            .expect("Invalid HTTP resource")
            .to_string();
        let protocol = HttpProtocol::from(first_line_parts.next().expect("Invalid HTTP protocol"));

        let mut headers = HashMap::new();
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            let mut header_parts = line.split(':');
            let header_name = header_parts
                .next()
                .expect("Invalid HTTP header")
                .trim()
                .to_string();
            let header_value = header_parts
                .next()
                .expect("Invalid HTTP header value")
                .trim()
                .to_string();
            headers.insert(header_name, header_value);
        }

        let body = lines.collect::<Vec<&str>>().join("\r\n");

        HttpRequest {
            method,
            resource,
            protocol,
            headers,
            body,
            params: None,
        }
    }
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
