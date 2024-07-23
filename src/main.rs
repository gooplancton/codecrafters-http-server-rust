mod handler;
mod response;
use std::net::TcpListener;

use handler::handler;

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                handler(&mut stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
