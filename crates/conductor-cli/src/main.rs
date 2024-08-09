use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080")?;

    loop {
        let mut buffer = [0; 4]; // 4 bytes for f32

        socket.recv_from(&mut buffer)?;

        println!("received: {}", f32::from_ne_bytes(buffer))
    }
}
