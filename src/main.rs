mod handlers;
mod request;
mod response;
mod router;

use crate::{
    request::HttpRequestReader,
    response::{HttpResponseBuilder, HttpResponseWriter},
};
use router::{HttpRouter, SimpleRouter};
use std::net::TcpListener;
use std::{io::Write, net::TcpStream};

pub fn dispatch(router: &impl HttpRouter, stream: &mut TcpStream) {
    let req = stream.read_http_req();
    if req.is_err() {
        println!(
            "Received invalid request: {:?}",
            &req.err().unwrap().message()
        );
        return;
    }

    let req = req.unwrap();
    let handler = router.get_handler(&req.target);
    let res = handler.map(|handler| handler(req)).unwrap_or({
        HttpResponseBuilder::default()
            .status(404, Some("Not Found"))
            .build()
    });

    let _ = stream.write_http_res(res);
}

fn main() {
    println!("Logs from your program will appear here!");

    let router = SimpleRouter {};
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                dispatch(&router, &mut stream);
                let _ = stream.flush();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
