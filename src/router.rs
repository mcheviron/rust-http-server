use crate::request::{HttpMethod, HttpRequest};
use crate::response::HttpResponse;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

type HandlerFn = fn(HttpRequest) -> HttpResponse;

#[derive(Clone)]
pub struct Route {
    pub path: String,
    pub params: Option<Vec<String>>,
}

impl Route {
    pub fn new(path: &str) -> Self {
        // Check if the path contains both '{' and '}' characters
        // If it doesn't, return a new instance of Self with the path as a string and no params
        if !path.contains('{') || !path.contains('}') {
            return Self {
                path: path.to_string(),
                params: None,
            };
        }

        // Create a new empty vector to store the parameter names
        let mut params = Vec::new();
        // Create a new empty string to store the cleaned path
        let mut clean_path = String::new();
        // Split the path string by '/' into separate parts
        let parts = path.split('/');

        // Iterate over each part of the path
        for part in parts {
            // Check if the part starts with '{' and ends with '}'
            // If it does, it's a parameter
            if part.starts_with('{') && part.ends_with('}') {
                // Extract the parameter name by removing the first and last characters ('{' and '}')
                // and convert it to a string, then push it to the params vector
                params.push(part[1..part.len() - 1].to_string());
                // Add a '/' to the clean_path string
                clean_path.push('/');
                // Add a '{}' placeholder to the clean_path string
                clean_path.push_str("{}");
            } else {
                // If the part is not a parameter, add a '/' to the clean_path string
                clean_path.push('/');
                // Add the part itself to the clean_path string
                clean_path.push_str(part);
            }
        }

        // Return a new instance of Self with the cleaned path and the extracted parameters
        Self {
            path: clean_path,
            params: Some(params),
        }
    }

    fn matches(&self, path: &str) -> Option<HashMap<String, String>> {
        // Check if the `params` field is `Some`. If it's `None`, return `None` early.
        // The `?` operator is used for early returns in case of `None`.
        self.params.as_ref()?;

        // Create a new empty `HashMap` to store the extracted parameter values.
        let mut extracted_params = HashMap::new();
        // Split the `path` field of `self` by '/' to get an iterator over the route segments.
        let mut route_parts = self.path.split('/');
        // Split the input `path` by '/' to get an iterator over the request path segments.
        let mut req_parts = path.split('/');

        // Start an infinite loop to iterate over the route and request path segments.
        loop {
            // Use pattern matching to compare the next segments from `route_parts` and `req_parts`.
            match (route_parts.next(), req_parts.next()) {
                // If the route segment is "{}" and the request segment exists,
                // it means we have a parameter to extract.
                (Some("{}"), Some(value)) => {
                    // Extract the parameter value.
                    if let Some(params) = &self.params {
                        // Get the parameter name from `self.params` based on the current number of extracted parameters.
                        // If the index is out of bounds, use an empty string as the parameter name.
                        let param_name = params
                            .get(extracted_params.len())
                            .cloned()
                            .unwrap_or_default();
                        // Insert the parameter name and value into the `extracted_params` map.
                        extracted_params.insert(param_name, value.to_string());
                    }
                }
                // If the route segment and request segment are equal (excluding "{}"),
                // continue to the next iteration of the loop.
                (Some(route_seg), Some(req_seg)) if route_seg == req_seg => {}
                // If both `route_parts` and `req_parts` are exhausted (i.e., `None`),
                // return the extracted parameters wrapped in `Some`.
                (None, None) => return Some(extracted_params),
                // If none of the above patterns match, it means the route doesn't match the request path.
                // Return `None` to indicate no match.
                _ => return None,
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
                    // Route matched, store parameters in request
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
