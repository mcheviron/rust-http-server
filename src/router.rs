use crate::request::{HttpMethod, HttpRequest};
use crate::response::HttpResponse;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

type HandlerFn = fn(HttpRequest) -> HttpResponse;

#[derive(Debug, Clone)]
pub struct Route {
    pub path: String,
    pub params: Option<Vec<String>>,
}

impl Route {
    pub fn new(path: &str) -> Self {
        if !path.contains('{') || !path.contains('}') {
            return Self {
                path: path.to_string(),
                params: None,
            };
        }

        let mut params = Vec::new();
        let mut clean_path = String::new();
        let parts = path.split('/');

        for (index, part) in parts.enumerate() {
            if part.starts_with('{') && part.ends_with('}') {
                let param_name = &part[1..part.len() - 1];
                params.push(param_name.to_string());
                if index > 0 {
                    clean_path.push('/');
                }
                clean_path.push_str("{}");
            } else {
                if index > 0 {
                    clean_path.push('/');
                }
                clean_path.push_str(part);
            }
        }

        Self {
            path: clean_path,
            params: Some(params),
        }
    }

    fn matches(&self, path: &str) -> Option<HashMap<String, String>> {
        let mut extracted_params = HashMap::new();
        let mut route_parts = self.path.split('/');
        let mut req_parts = path.split('/');

        loop {
            match (route_parts.next(), req_parts.next()) {
                (Some("{}"), Some(value)) => {
                    if let Some(params) = &self.params {
                        let param_name = params
                            .get(extracted_params.len())
                            .cloned()
                            .unwrap_or_default();
                        extracted_params.insert(param_name, value.to_string());
                    }
                }
                (Some(route_seg), Some(req_seg)) if route_seg == req_seg => {}
                (None, None) => {
                    return Some(extracted_params);
                }
                _ => {
                    return None;
                }
            }
        }
    }
}

pub struct Router {
    routes: HashMap<(HttpMethod, Route), HandlerFn>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }

    fn add_route(&mut self, method: HttpMethod, path: &str, handler: HandlerFn) {
        let route = Route::new(path);
        self.routes.insert((method, route), handler);
    }

    pub fn get(&mut self, path: &str, handler: HandlerFn) {
        self.add_route(HttpMethod::Get, path, handler);
    }

    pub fn post(&mut self, path: &str, handler: HandlerFn) {
        self.add_route(HttpMethod::Post, path, handler);
    }

    pub fn put(&mut self, path: &str, handler: HandlerFn) {
        self.add_route(HttpMethod::Put, path, handler);
    }

    pub fn delete(&mut self, path: &str, handler: HandlerFn) {
        self.add_route(HttpMethod::Delete, path, handler);
    }

    pub fn handle_request(&self, mut request: HttpRequest) -> HttpResponse {
        for ((method, route), handler) in &self.routes {
            if *method == request.method {
                if let Some(params) = route.matches(&request.resource) {
                    request.params = Some(params);
                    return handler(request);
                }
            }
        }
        HttpResponse::NotFound
    }
}

impl PartialEq for HttpMethod {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl Eq for HttpMethod {}

impl Hash for HttpMethod {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state);
    }
}

impl PartialEq for Route {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Eq for Route {}

impl Hash for Route {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}
