use core::panic;
use std::{collections::HashMap, str::FromStr};

use crate::{
    request::{HttpMethod, HttpRequest},
    response::{HttpResponse, HttpResponseBuilder},
};
use regex::Regex;

pub trait HttpRouter {
    fn dispatch(self: &Self, req: HttpRequest) -> HttpResponse;
}

type HandlerInner = fn(request: HttpRequest, params: HashMap<String, String>) -> HttpResponse;

#[derive(Clone, Debug)]
pub struct HttpRegexEndpoint {
    method: HttpMethod,
    path: Regex,
    handler: HandlerInner,
}

impl HttpRegexEndpoint {
    pub fn new(method: HttpMethod, path_str: impl AsRef<str>, handler: HandlerInner) -> Self {
        let path_str = path_str.as_ref();

        if !path_str.starts_with("/") {
            panic!("Paths must start with a '/'")
        }

        let mut path = String::from("^");
        let mut segments = path_str.split("/");
        let _ = segments.next();
        segments.for_each(|segment| {
            path.push_str("\\/");
            if segment.starts_with(":") {
                let segment = segment.strip_prefix(":").unwrap();
                let capturing_group = format!("(?<{}>\\w+)", segment);
                path.push_str(&capturing_group);
            } else {
                path.push_str(segment);
            }
        });

        path.push_str("$");
        let path = Regex::from_str(&path).unwrap();

        HttpRegexEndpoint {
            method,
            path,
            handler,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RegexRouter {
    pub endpoints: Vec<HttpRegexEndpoint>,
}

impl RegexRouter {
    pub fn extract_params(self: &Self, path: &Regex, target: &String) -> HashMap<String, String> {
        let mut params = HashMap::<String, String>::new();
        let captures = path.captures(&target);
        captures.map(|captures| {
            captures
                .iter()
                .zip(path.capture_names())
                .filter_map(|(capture, name)| Option::zip(name, capture))
                .for_each(|(name, capture)| {
                    params.insert(name.to_string(), capture.as_str().into());
                });
        });

        params
    }
}

impl HttpRouter for RegexRouter {
    fn dispatch(self: &Self, req: HttpRequest) -> HttpResponse {
        for endpoint in &self.endpoints {
            if endpoint.method == req.method && endpoint.path.is_match(req.target.as_ref()) {
                let params = self.extract_params(&endpoint.path, &req.target);

                return (endpoint.handler)(req, params);
            }
        }

        HttpResponseBuilder::default()
            .status(404, Some("Not Found"))
            .build()
    }
}

