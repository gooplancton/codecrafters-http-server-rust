use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    net::TcpStream,
};

#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub target: String,
    pub headers: HashMap<String, String>,
}

#[derive(Debug)]
pub struct HttpRequestParsingError(String);

impl HttpRequestParsingError {
    pub fn message(self: &Self) -> String {
        self.0.clone()
    }
}

pub struct HttpRequestBuilder {
    method: HttpMethod,
    target: String,
    headers: HashMap<String, String>,
}

impl HttpRequestBuilder {
    pub fn from_request_line(
        request_line: impl AsRef<str>,
    ) -> Result<Self, HttpRequestParsingError> {
        let mut segments = request_line.as_ref().split(" ");
        let method = match segments.next() {
            Some("GET") => Ok(HttpMethod::GET),
            Some("POST") => Ok(HttpMethod::POST),
            Some("PUT") => Ok(HttpMethod::PUT),
            Some("DELETE") => Ok(HttpMethod::DELETE),
            Some("HEAD") => Ok(HttpMethod::HEAD),
            Some("OPTIONS") => Ok(HttpMethod::OPTIONS),
            None => Err(HttpRequestParsingError("Missing HTTP verb".to_string())),
            Some(method) => Err(HttpRequestParsingError(format!(
                "Invalid HTTP verb: {}",
                method
            ))),
        }?;

        let target = match segments.next() {
            Some(target) if target.starts_with("/") => Ok(target.to_string()),
            Some(_) => Err(HttpRequestParsingError(
                "Request target must start with /".to_string(),
            )),
            None => Err(HttpRequestParsingError(
                "Missing request target".to_string(),
            )),
        }?;

        let version = segments.next();
        if version != Some("HTTP/1.1\r\n") {
            return Err(HttpRequestParsingError(format!(
                "Invalid HTTP version or missing CRLF sequence: {}",
                version.unwrap_or("")
            )));
        }

        Ok(HttpRequestBuilder {
            method,
            target,
            headers: HashMap::default(),
        })
    }

    #[allow(dead_code)]
    pub fn header(
        mut self: Self,
        header_name: impl AsRef<str>,
        header_value: impl AsRef<str>,
    ) -> Self {
        self.headers
            .insert(header_name.as_ref().into(), header_value.as_ref().into());

        self
    }

    pub fn build(self: Self) -> HttpRequest {
        HttpRequest {
            method: self.method,
            target: self.target,
            headers: self.headers,
        }
    }
}

pub trait HttpRequestReader {
    fn read_http_req(self: &Self) -> Result<HttpRequest, HttpRequestParsingError>;
}

impl HttpRequestReader for TcpStream {
    fn read_http_req(self: &Self) -> Result<HttpRequest, HttpRequestParsingError> {
        let mut reader = BufReader::new(self);

        let mut request_line = String::new();
        reader
            .read_line(&mut request_line)
            .map_err(|err| HttpRequestParsingError(err.to_string()))?;

        let builder = HttpRequestBuilder::from_request_line(request_line)?;

        Ok(builder.build())
    }
}
