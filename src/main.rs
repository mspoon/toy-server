use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let listener = TcpListener::bind("localhost:8080").unwrap();

    listener.accept();

    drop(listener);
}
