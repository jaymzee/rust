use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Listening on TCP {}", listener.local_addr()?);

    foo().await;
    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            process(socket).await
        });
    }
}

async fn foo() {
    println!("hey there");
}

async fn process(mut socket: TcpStream) {
    println!("hi");
    socket.write_all(b"Hello").await
        .expect("cannot write to socket");
}
