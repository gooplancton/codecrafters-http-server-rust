use std::{io::Write, net::TcpStream};

use crate::{
    request::HttpRequestReader,
    response::{HttpResponse, HttpResponseWriter},
};

pub fn handler(stream: &mut TcpStream) {
    let req = stream.read_http_req();
    let res = match req {
        Ok(req) if req.target == "/" => HttpResponse::ok(),
        Err(err) => HttpResponse::bad_request(err.message()),
        _ => HttpResponse::not_found()
    };

    let _ = stream.write_http_res(res);
    let _ = stream.flush();
}
