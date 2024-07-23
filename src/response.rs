use std::{io::Write, net::TcpStream};

pub struct HTTPResponse {
    pub status_code: usize,
    pub status_message: Option<String>,
}

impl HTTPResponse {
    pub fn ok(status_message: Option<String>) -> Self {
        HTTPResponse {
            status_code: 200,
            status_message,
        }
    }
}

pub trait HTTPResponseWriter {
    fn write_http_res(self: &mut Self, res: HTTPResponse) -> std::result::Result<usize, std::io::Error>;
}

impl HTTPResponseWriter for TcpStream {
    fn write_http_res(self: &mut Self, res: HTTPResponse) -> std::result::Result<usize, std::io::Error> {
        let status_line = format!("HTTP/1.1 {} {}\r\n", res.status_code, res.status_message.unwrap_or("".to_owned()));

        let bytes = self.write(&status_line.into_bytes())?;

        Ok(bytes)
    }
}
