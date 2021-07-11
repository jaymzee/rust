use std::env;
use std::process;
use std::net::TcpStream;
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: tcp-client MESSAGE COUNT INTERVAL");
        process::exit(1);
    }

    let count = match args[2].parse() {
        Ok(n) if n > 0 => n,
        _ => {
            eprintln!("count must be at least 1");
            process::exit(1);
        }
    };

    let interval = match args[3].parse() {
        Ok(n) if n > 0 => n,
        _ => {
            eprintln!("interval must be at least 1 (milliseconds)");
            process::exit(1);
        }
    };


    send_message("localhost:3333", &args[1], count, interval);
}

fn send_message(server: &str, msg: &str, count: u32, interval: u64) {
    let mut stream = TcpStream::connect(server)
        .expect("Failed to connect to server");

    println!("Successfully connected to server");
    println!("sending {} {} times at {} ms interval", msg, count, interval);

    let tx_data = msg.as_bytes();

    for _ in 0..count {
        stream.write(tx_data).unwrap();
        println!("sent: {} bytes, \"{}\"", tx_data.len(), msg);

        let mut rx_buffer = [0; 1024]; // 1k buffer
        if let Ok(rx_len) = stream.read(&mut rx_buffer) {
            let s = String::from_utf8_lossy(&rx_buffer);
            println!("received: {} bytes, \"{}\"", rx_len, s);
        }
        thread::sleep(Duration::from_millis(interval));
    }
}
