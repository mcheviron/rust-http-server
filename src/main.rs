use std::{
    fmt,
    io::{Read, Write},
    net::TcpListener,
};

enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

enum HttpProtocol {
    Http11,
}

struct HttpRequest {
    method: HttpMethod,
    resource: String,
    protocol: HttpProtocol,
    headers: Vec<(String, String)>,
    body: String,
}

enum HttpResponse {
    Ok,
    NotFound,
}

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for mut stream in listener.incoming().flatten() {
        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer).expect("Failed to read request");
        let request = String::from_utf8_lossy(&buffer[..bytes_read]);
        let http_request = handle_request(&request);

        let response = match http_request {
            Ok(req) => {
                if req.resource != "/" {
                    HttpResponse::NotFound
                } else {
                    HttpResponse::Ok
                }
            }
            Err(_) => HttpResponse::NotFound,
        };

        stream.write_all(response.to_string().as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn handle_request(request: &str) -> Result<HttpRequest, String> {
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

    let mut headers = Vec::new();
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
        headers.push((header_name, header_value));
    }

    let body = lines.collect::<Vec<&str>>().join("\r\n");

    Ok(HttpRequest {
        method,
        resource,
        protocol,
        headers,
        body,
    })
}

impl fmt::Display for HttpResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpResponse::Ok => write!(f, "HTTP/1.1 200 OK\r\n\r\n"),
            HttpResponse::NotFound => write!(f, "HTTP/1.1 404 Not Found\r\n\r\n"),
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
