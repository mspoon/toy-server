use std::net::{TcpListener, TcpStream};
use std::thread;

use std::io::Read;
use std::io::Write;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0u8; 2046];
    let num_read = stream.read(&mut buffer).unwrap_or(0);

    if num_read > 0 {
        let _ = stream.write(&mut &buffer[0 .. num_read]);
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
