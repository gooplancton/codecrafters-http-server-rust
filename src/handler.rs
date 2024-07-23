use std::net::TcpStream;

use crate::{
    request::HttpRequestReader,
    response::{HttpResponse, HttpResponseWriter},
};

pub fn handler(stream: &mut TcpStream) {
    let req = stream.read_http_req();
    let res = match req {
        Ok(req) => {
            if req.target == "/" {
                HttpResponse::ok()
            } else {
                HttpResponse::not_found()
            }
        }
        Err(err) => HttpResponse::bad(err.message()),
    };

    let _ = stream.write_http_res(res);
}
