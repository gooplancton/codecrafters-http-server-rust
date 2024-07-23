use std::{collections::HashMap, io::Write, net::TcpStream};

#[derive(Debug)]
pub struct HttpResponse {
    pub status_code: usize,
    pub status_message: Option<String>,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

pub struct HttpResponseBuilder {
    _status_code: usize,
    _status_message: Option<String>,
    _headers: HashMap<String, String>,
    _body: Option<String>,
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
        self._headers
            .insert(header_name.as_ref().into(), header_value.as_ref().into());

        self
    }

    pub fn body(mut self: Self, body: impl AsRef<str>) -> Self {
        self._body = Some(body.as_ref().into());

        self
    }

    pub fn build(mut self: Self) -> HttpResponse {
        if self._body.is_some() {
            let content_length = &self._body.as_ref().unwrap().len();
            self._headers
                .insert("Content-Length".into(), content_length.to_string());
        }

        if self._headers.get("Content-Type").is_none() {
            self._headers
                .insert("Content-Type".into(), "text/plain".into());
        }

        HttpResponse {
            status_code: self._status_code,
            status_message: self._status_message,
            headers: self._headers,
            body: self._body,
        }
    }
}

impl Default for HttpResponseBuilder {
    fn default() -> Self {
        HttpResponseBuilder {
            _status_code: 200,
            _status_message: None,
            _headers: HashMap::default(),
            _body: None,
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
        let mut bytes = 0;

        let status_line = format!(
            "HTTP/1.1 {} {}\r\n",
            res.status_code,
            res.status_message.unwrap_or("".to_owned())
        );
        bytes += self.write(&status_line.into_bytes())?;

        for (header_name, header_value) in res.headers.into_iter() {
            let header = format!("{}: {}\r\n", header_name, header_value);
            bytes += self.write(&header.into_bytes())?;
        }

        bytes += self.write(b"\r\n")?;

        if res.body.is_some() {
            bytes += self.write(&res.body.unwrap().into_bytes())?;
        }

        Ok(bytes)
    }
}
