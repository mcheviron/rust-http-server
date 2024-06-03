#![allow(dead_code)]

use std::{io::Read, net::TcpListener};

mod request;
mod response;
mod router;

use request::{parse_request, HttpRequest};
use response::{send_response, ContentType, HttpResponse};
use router::Router;

fn handle_home(_request: HttpRequest) -> HttpResponse {
    HttpResponse::Ok(ContentType::PlainText("".to_string()))
}

fn handle_echo(request: HttpRequest) -> HttpResponse {
    if let Some(params) = request.params {
        if let Some(str_value) = params.get("str") {
            return HttpResponse::Ok(ContentType::PlainText(str_value.to_string()));
        }
    }
    HttpResponse::NotFound
}

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").expect("Failed to bind to address");
    println!("Listening on http://127.0.0.1:4221");
    let mut router = Router::new();

    router.get("/", handle_home);
    router.get("/echo/{str}", handle_echo);

    for mut stream in listener.incoming().flatten() {
        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer).expect("Failed to read request");
        let request = String::from_utf8_lossy(&buffer[..bytes_read]);
        let http_request = parse_request(&request);

        let response = match http_request {
            Ok(req) => router.handle_request(req),
            Err(_) => HttpResponse::NotFound,
        };

        send_response(&mut stream, response);
    }
}
