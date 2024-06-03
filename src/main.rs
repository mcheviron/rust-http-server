use request::HttpRequest;
use response::{ContentType, HttpResponse};
use router::Router;
use std::net::TcpListener;
use std::path::Path;
use std::{env, fs};

mod request;
mod response;
mod router;

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

fn handle_user_agent(request: HttpRequest) -> HttpResponse {
    if let Some(user_agent) = request.headers.get("User-Agent") {
        HttpResponse::Ok(ContentType::PlainText(user_agent.to_string()))
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
