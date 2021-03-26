use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("192.168.1.22:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 10244];
    stream.read(&mut buffer).unwrap();

    let set_gpio = b"GET /gpio_1 HTTP/1.1\r\n";
    let reset_gpio = b"GET /gpio_0 HTTP/1.1\r\n";

    if buffer.starts_with(set_gpio) {
        println!("Set gpio.");
    } 

    if buffer.starts_with(reset_gpio) {
        println!("Reset gpio.");
    } 

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}