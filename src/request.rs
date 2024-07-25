use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    net::TcpStream,
};

use bytes::Bytes;

use crate::shared::HttpEncodingScheme;

#[derive(Debug, Clone, PartialEq)]
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
    pub body: Option<Bytes>,
    pub accepted_encodings: Vec<HttpEncodingScheme>,
}

#[derive(Debug)]
pub struct HttpRequestParsingError(String);

impl HttpRequestParsingError {
    pub fn message(self: &Self) -> String {
        self.0.clone()
    }
}

#[derive(Debug)]
pub struct HttpRequestBuilder {
    _method: HttpMethod,
    _target: String,
    _headers: HashMap<String, String>,
    _body: Option<Bytes>,
    _accepted_encodings: Vec<HttpEncodingScheme>,
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
            _method: method,
            _target: target,
            _headers: HashMap::default(),
            _body: None,
            _accepted_encodings: vec![],
        })
    }

    pub fn header(self: &mut Self, header_name: impl AsRef<str>, header_value: impl AsRef<str>) {
        self._headers.insert(
            header_name.as_ref().to_lowercase(),
            header_value.as_ref().into(),
        );
    }

    pub fn body(self: &mut Self, body: impl Into<Bytes>) {
        self._body = Some(body.into());
    }

    pub fn accept_encoding(self: &mut Self, encoding_name: impl AsRef<str>) {
        let encoding_scheme = match encoding_name.as_ref() {
            "gzip" => Some(HttpEncodingScheme::Gzip),
            _ => None,
        };

        if let Some(encoding_scheme) = encoding_scheme {
            self._accepted_encodings.push(encoding_scheme);
        }
    }

    pub fn build(self: Self) -> HttpRequest {
        HttpRequest {
            method: self._method,
            target: self._target,
            headers: self._headers,
            body: self._body,
            accepted_encodings: self._accepted_encodings,
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

        let mut builder = HttpRequestBuilder::from_request_line(request_line)?;

        let mut content_length = 0;
        loop {
            let mut header_line = String::new();
            reader
                .read_line(&mut header_line)
                .map_err(|err| HttpRequestParsingError(err.to_string()))?;

            header_line = header_line
                .strip_suffix("\r\n")
                .ok_or(HttpRequestParsingError(
                    "Missing CRLF sequence after header".into(),
                ))?
                .to_owned();

            if header_line.len() == 0 {
                break;
            }

            let (header_name, header_value) = header_line
                .split_once(": ")
                .ok_or(HttpRequestParsingError("Incorrect header format".into()))?;

            let header_name = header_name.to_lowercase();
            let header_value = header_value.strip_suffix("\r\n").unwrap_or(header_value);

            if header_name == "content-length" {
                content_length = str::parse::<usize>(header_value)
                    .map_err(|_| HttpRequestParsingError("Invalid content-length header".into()))?;
            } else if header_name == "accept-encoding" {
                header_value
                    .split(" ")
                    .for_each(|encoding_name| builder.accept_encoding(encoding_name));
            }

            builder.header(header_name, header_value);
        }

        if content_length > 0 {
            let mut body = vec![0u8; content_length];

            reader
                .read_exact(&mut body)
                .map_err(|err| HttpRequestParsingError(err.to_string()))?;

            builder.body(body);
        }

        Ok(builder.build())
    }
}
