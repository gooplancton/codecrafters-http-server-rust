use std::{collections::HashMap, io::Write, net::TcpStream};

#[derive(Debug)]
pub struct HttpResponse {
    pub status_code: usize,
    pub status_message: Option<String>,
    pub headers: HashMap<String, String>,
}

impl HttpResponse {
    pub fn ok() -> Self {
        HttpResponse {
            status_code: 200,
            status_message: Some("OK".to_owned()),
            headers: HashMap::default(),
        }
    }

    pub fn bad_request(status_message: impl AsRef<str>) -> Self {
        HttpResponse {
            status_code: 400,
            status_message: Some(status_message.as_ref().to_owned()),
            headers: HashMap::default(),
        }
    }

    pub fn not_found() -> Self {
        HttpResponse {
            status_code: 404,
            status_message: Some("Not Found".to_owned()),
            headers: HashMap::default(),
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

        Ok(bytes)
    }
}
