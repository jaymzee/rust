use web_mt::ThreadPool;
use std::fs;
use std::thread;
use std::time::Duration;
use std::io::{Read, Write};
use std::net::{TcpStream, TcpListener};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878")
        .expect("Could not listen on 7878");
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; // 1k buffer

    stream.read(&mut buffer)
        .expect("failed to read stream");
    println!("Request: {}", String::from_utf8_lossy(&buffer));

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename)
        .expect("failed to read html file");
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes())
        .expect("failed to write stream");
    stream.flush().unwrap();
}
