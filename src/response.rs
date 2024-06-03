use std::fmt;
use std::io::Write;

pub enum HttpResponse {
    Ok(ContentType),
    NotFound,
}

pub enum ContentType {
    PlainText(String),
    Html(String),
}

pub fn send_response<W: Write>(stream: &mut W, response: HttpResponse) {
    stream.write_all(response.to_string().as_bytes()).unwrap();
    stream.flush().expect("Failed to flush response");
}

impl fmt::Display for HttpResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpResponse::Ok(content_type) => {
                let (content_type_str, body) = match content_type {
                    ContentType::PlainText(text) => ("text/plain", text),
                    ContentType::Html(html) => ("text/html", html),
                };
                let content_length = body.len();
                write!(
                    f,
                    "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
                    content_type_str, content_length, body
                )
            }
            HttpResponse::NotFound => write!(f, "HTTP/1.1 404 Not Found\r\n\r\n"),
        }
    }
}
