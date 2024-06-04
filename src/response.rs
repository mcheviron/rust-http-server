use std::fmt;
use std::io::Write;

pub enum HttpResponse {
    Ok(ContentType, Option<ContentEncoding>),
    Created,
    NotFound,
}

pub enum ContentEncoding {
    Gzip,
}

pub enum ContentType {
    PlainText(String),
    OctetStream(Vec<u8>),
}

pub fn send_response<W: Write>(stream: &mut W, response: HttpResponse) {
    stream.write_all(response.to_string().as_bytes()).unwrap();
    stream.flush().expect("Failed to flush response");
}

impl fmt::Display for HttpResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpResponse::Ok(content_type, content_encoding) => {
                let (content_type_str, body) = match content_type {
                    ContentType::PlainText(text) => ("text/plain", text.as_str()),
                    ContentType::OctetStream(data) => (
                        "application/octet-stream",
                        std::str::from_utf8(data).unwrap(),
                    ),
                };

                let content_encoding_str = match content_encoding {
                    Some(ContentEncoding::Gzip) => "Content-Encoding: gzip\r\n",
                    None => "",
                };

                write!(
                    f,
                    "HTTP/1.1 200 OK\r\nContent-Type: {}\r\n{}Content-Length: {}\r\n\r\n{}",
                    content_type_str,
                    content_encoding_str,
                    body.len(),
                    body
                )
            }
            HttpResponse::Created => write!(f, "HTTP/1.1 201 Created\r\n\r\n"),
            HttpResponse::NotFound => write!(f, "HTTP/1.1 404 Not Found\r\n\r\n"),
        }
    }
}
