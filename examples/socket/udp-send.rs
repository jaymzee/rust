use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    socket.connect("127.0.0.1:514")?;

    let message = "<14>rust greetings from rust!";
    socket.send(message.as_bytes())?;

    Ok(())
}
