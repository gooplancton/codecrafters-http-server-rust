use crate::shared::HttpEncodingScheme;
use std::{collections::HashMap, io::Write, net::TcpStream};

use bytes::Bytes;
use libdeflater::{CompressionLvl, Compressor};

#[derive(Debug)]
pub struct HttpResponse {
    pub status_code: usize,
    pub status_message: Option<String>,
    pub headers: HashMap<String, String>,
    pub body: Option<Bytes>,
}

pub struct HttpResponseBuilder {
    _status_code: usize,
    _status_message: Option<String>,
    _headers: HashMap<String, String>,
    _body: Option<Bytes>,
    _encoding: HttpEncodingScheme,
}

#[allow(dead_code)]
impl HttpResponseBuilder {
    pub fn status(mut self: Self, code: usize, message: Option<impl AsRef<str>>) -> Self {
        self._status_code = code;
        self._status_message = message.map(|message| message.as_ref().into());

        self
    }

    pub fn header(
        mut self: Self,
        header_name: impl AsRef<str>,
        header_value: impl AsRef<str>,
    ) -> Self {
        let header_name = header_name.as_ref().to_lowercase().into();
        let header_value = header_value.as_ref().into();
        self._headers.insert(header_name, header_value);

        self
    }

    pub fn encode(mut self: Self, accepted_schemes: Vec<HttpEncodingScheme>) -> Self {
        if accepted_schemes.contains(&HttpEncodingScheme::Gzip) {
            self._encoding = HttpEncodingScheme::Gzip;
            self._headers
                .insert("content-encoding".into(), "gzip".into());
        }

        self
    }

    pub fn body(mut self: Self, body: impl Into<Bytes>) -> Self {
        let body: Bytes = body.into();
        self._body = Some(body);

        self
    }

    pub fn build(mut self: Self) -> HttpResponse {
        if self._body.is_none() {
            return HttpResponse {
                status_code: self._status_code,
                status_message: self._status_message,
                headers: self._headers,
                body: self._body,
            };
        }

        let mut body = self._body.unwrap();
        let content_length = &body.len();
        self._headers
            .insert("content-length".into(), content_length.to_string());

        if self._headers.get("content-type").is_none() {
            self._headers
                .insert("content-type".into(), "text/plain".into());
        }

        body = match self._encoding {
            HttpEncodingScheme::None => body,
            HttpEncodingScheme::Gzip => {
                let (new_content_length, new_body) = gzip_encode(*content_length, body);
                self._headers
                    .insert("content-length".into(), new_content_length.to_string());

                new_body
            }
        };

        return HttpResponse {
            status_code: self._status_code,
            status_message: self._status_message,
            headers: self._headers,
            body: Some(body),
        };
    }
}

impl Default for HttpResponseBuilder {
    fn default() -> Self {
        HttpResponseBuilder {
            _status_code: 200,
            _status_message: None,
            _headers: HashMap::default(),
            _body: None,
            _encoding: HttpEncodingScheme::None,
        }
    }
}

pub trait HttpResponseWriter {
    fn write_http_res(
        self: &mut Self,
        res: HttpResponse,
    ) -> std::result::Result<usize, std::io::Error>;
}

impl HttpResponseWriter for TcpStream {
    fn write_http_res(
        self: &mut Self,
        res: HttpResponse,
    ) -> std::result::Result<usize, std::io::Error> {
        let mut n_bytes = 0;

        let status_line = format!(
            "HTTP/1.1 {} {}\r\n",
            res.status_code,
            res.status_message.unwrap_or("".to_owned())
        );
        n_bytes += self.write(&status_line.into_bytes())?;

        for (header_name, header_value) in res.headers.into_iter() {
            let header = format!("{}: {}\r\n", header_name, header_value);
            n_bytes += self.write(&header.into_bytes())?;
        }

        n_bytes += self.write(b"\r\n")?;

        if let Some(body) = res.body {
            n_bytes += self.write(&body)?;
        }

        Ok(n_bytes)
    }
}

fn gzip_encode(content_length: usize, payload: impl Into<Bytes>) -> (usize, Bytes) {
    let mut decompressor = Compressor::new(CompressionLvl::fastest());
    let compression_bound = decompressor.gzip_compress_bound(content_length);
    let mut out = vec![0u8; compression_bound];
    let new_content_length = decompressor
        .gzip_compress(&payload.into(), &mut out)
        .unwrap();

    (new_content_length, out.into())
}
