use std::net::TcpStream;

use crate::response::{HTTPResponse, HTTPResponseWriter};

pub fn handler(stream: &mut TcpStream) {
    let res = HTTPResponse::ok(Some("OK".to_owned()));
    let _ = stream.write_http_res(res);
}
