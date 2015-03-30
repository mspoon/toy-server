use std::net::{TcpListener, TcpStream};
use std::thread;

use std::io::Read;
use std::io::Write;

fn handle_client(mut stream: TcpStream) {
    let mut buf = String::new();
    let num_read = stream.read_to_string(&mut buf).unwrap_or(0);

    if num_read > 0 {
        let _ = stream.write_fmt(format_args!("Hello, {}", buf));
    }
}

fn main() {
    let listener = TcpListener::bind("localhost:8080").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| { handle_client(stream) } );
            },
            Err(e) => {
                println!("ERROR: {}", e);
            },
        }
    }

    drop(listener);
}
