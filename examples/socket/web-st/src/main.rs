use std::fs;
use std::io::{Read, Write};
use std::net::{TcpStream, TcpListener};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878")
        .expect("Could not listen on 7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; // 1k buffer
    let contents = fs::read_to_string("hello.html")
        .expect("failed to read html file");
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.read(&mut buffer)
        .expect("failed to read stream");

    println!("Request: {}", String::from_utf8_lossy(&buffer));

    stream.write(response.as_bytes())
        .expect("failed to write stream");

    stream.flush().unwrap();
}
