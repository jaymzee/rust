use std::thread;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50];  // using 50 byte buffer
    let peer = stream.peer_addr().unwrap();
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything
            if size > 0 {
                println!("echoing {} bytes from {}", size, peer);
                stream.write(&data[0..size])
                    .expect("write to stream failed");
            }
            size > 0
        }
        Err(_) => {
            print!("an error occured - ");
            false
        }
    } {}
    println!("terminating connection with {}", peer);
}

fn main() {
    let port = 3333;
    let address = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&address)
        .expect("cannot bind to socket");
    // accept connections and process them, spawing a new thread for each one
    println!("server listening on {}", address);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("new connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    // connection succeeded
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                // connection failed
            }
        }
    }
}
