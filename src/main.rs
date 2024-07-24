mod handlers;
mod request;
mod response;
mod router;

use crate::{
    request::HttpRequestReader,
    response::HttpResponseWriter,
};
use handlers::{echo, home, user_agent};
use request::HttpMethod;
use router::{HttpRegexEndpoint, HttpRouter, RegexRouter};
use std::net::TcpListener;
use std::{io::Write, net::TcpStream, thread};

pub fn dispatch(router: impl HttpRouter, stream: &mut TcpStream) {
    let req = stream.read_http_req();
    if req.is_err() {
        println!( "Received invalid request: {:?}", &req.err().unwrap().message());

        return;
    }

    let req = req.unwrap();
    let res = router.dispatch(req);

    let _ = stream.write_http_res(res);
}

fn main() {
    println!("Logs from your program will appear here!");

    let router = RegexRouter {
        endpoints: vec![
            HttpRegexEndpoint::new(HttpMethod::GET, "/", home),
            HttpRegexEndpoint::new(HttpMethod::GET, "/echo/:message", echo),
            HttpRegexEndpoint::new(HttpMethod::GET, "/user-agent", user_agent)
        ]
    };
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let router = router.clone();
                let mut stream_handle = stream.try_clone().unwrap();
                let _ = thread::spawn(move || {
                    dispatch(router, &mut stream_handle);
                    let _ = stream.flush();
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
