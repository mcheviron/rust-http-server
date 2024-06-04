#![allow(dead_code)]

use request::HttpRequest;
use response::{ContentEncoding, ContentType, HttpResponse};
use router::Router;
use std::env;
use std::net::TcpListener;

mod request;
mod response;
mod router;

fn handle_home(_request: HttpRequest) -> HttpResponse {
    HttpResponse::Ok(ContentType::PlainText("".to_string()), None)
}

fn handle_echo(request: HttpRequest) -> HttpResponse {
    if let Some(params) = request.params {
        if let Some(str_value) = params.get("str") {
            let content_encoding = if request
                .headers
                .get("Accept-Encoding")
                .map_or(false, |v| v.contains("gzip"))
            {
                Some(ContentEncoding::Gzip)
            } else {
                None
            };
            return HttpResponse::Ok(
                ContentType::PlainText(str_value.to_string()),
                content_encoding,
            );
        }
    }
    HttpResponse::NotFound
}

fn handle_user_agent(request: HttpRequest) -> HttpResponse {
    if let Some(user_agent) = request.headers.get("User-Agent") {
        let content_encoding = if request
            .headers
            .get("Accept-Encoding")
            .map_or(false, |v| v.contains("gzip"))
        {
            Some(ContentEncoding::Gzip)
        } else {
            None
        };
        HttpResponse::Ok(
            ContentType::PlainText(user_agent.to_string()),
            content_encoding,
        )
    } else {
        HttpResponse::NotFound
    }
}

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").expect("Failed to bind to address");
    let args = env::args().collect::<Vec<String>>();
    let directory = args
        .iter()
        .position(|arg| arg == "--directory")
        .and_then(|index| args.get(index + 1))
        .cloned();

    let mut router = Router::new(listener, directory);

    println!("Listening on http://127.0.0.1:4221");

    router.get("/", handle_home);
    router.get("/user-agent", handle_user_agent);
    router.get("/echo/{str}", handle_echo);

    router.run();
}
