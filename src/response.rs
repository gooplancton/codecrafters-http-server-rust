use std::{collections::HashMap, io::Write, net::TcpStream};

pub struct HTTPResponse {
    pub status_code: usize,
    pub status_message: Option<String>,
    pub headers: HashMap<String, String>,
}

impl HTTPResponse {
    pub fn ok(status_message: Option<String>) -> Self {
        HTTPResponse {
            status_code: 200,
            status_message,
            headers: HashMap::default(),
        }
    }
}

pub trait HTTPResponseWriter {
    fn write_http_res(
        self: &mut Self,
        res: HTTPResponse,
    ) -> std::result::Result<usize, std::io::Error>;
}

impl HTTPResponseWriter for TcpStream {
    fn write_http_res(
        self: &mut Self,
        res: HTTPResponse,
    ) -> std::result::Result<usize, std::io::Error> {
        let mut bytes = 0;

        let status_line = format!(
            "HTTP/1.1 {} {}\r\n",
            res.status_code,
            res.status_message.unwrap_or("".to_owned())
        );
        bytes += self.write(&status_line.into_bytes())?;

        for (header_name, header_value) in res.headers.into_iter() {
            let header = format!("{}:{}\r\n", header_name, header_value);
            bytes += self.write(&header.into_bytes())?;
        }

        Ok(bytes)
    }
}
