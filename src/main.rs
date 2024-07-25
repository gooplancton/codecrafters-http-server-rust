mod handlers;
mod request;
mod response;
mod router;

use crate::{request::HttpRequestReader, response::HttpResponseWriter};
use handlers::{create_file, echo, get_file, home, user_agent};
use request::HttpMethod;
use router::{HttpRegexEndpoint, HttpRouter, RegexRouter};
use std::env::Args;
use std::env;
use std::net::TcpListener;
use std::{io::Write, net::TcpStream, thread};

pub fn dispatch(router: impl HttpRouter, mut stream: TcpStream) {
    let req = stream.read_http_req();
    if req.is_err() {
        println!(
            "Received invalid request: {:?}",
            &req.err().unwrap().message()
        );

        return;
    }

    let req = req.unwrap();
    let res = router.dispatch(req);

    let _ = stream.write_http_res(res);
    let _ = stream.flush();
}

fn main() {
    println!("Logs from your program will appear here!");

    let data_dir = parse_directory_flag(env::args()).unwrap_or(env!("PWD").to_string());
    env::set_var("DATA_DIR", data_dir);

    let router = RegexRouter {
        endpoints: vec![
            HttpRegexEndpoint::new(HttpMethod::GET, "/", home),
            HttpRegexEndpoint::new(HttpMethod::GET, "/echo/:message", echo),
            HttpRegexEndpoint::new(HttpMethod::GET, "/user-agent", user_agent),
            HttpRegexEndpoint::new(HttpMethod::GET, "/files/:filename", get_file),
            HttpRegexEndpoint::new(HttpMethod::POST, "/files/:filename", create_file),
        ],
    };

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let router = router.clone();
                let _ = thread::spawn(|| dispatch(router, stream));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}


fn parse_directory_flag(mut argv: Args) -> Option<String> {
    while let Some(arg) = argv.next() {
        if arg == "--directory" {
            return argv.next();
        }
    };

    None
}

