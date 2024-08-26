use conductor_core::nodes::{Node, SourcePort};
use std::net::UdpSocket;

pub trait UdpDeserializer {
    fn max_packet_size() -> usize;
    fn deserialize_packet(bytes: &[u8]) -> Self;
}

pub struct UdpReceiver<T: Clone + UdpDeserializer> {
    socket: UdpSocket,
    pub output: SourcePort<T>,
}

impl<T: Clone + UdpDeserializer> UdpReceiver<T> {
    pub fn new(addr: &str) -> Self {
        let socket = UdpSocket::bind(addr).unwrap();

        Self {
            socket,
            output: SourcePort::<T>::new(),
        }
    }
}

impl<T: Clone + UdpDeserializer> Node for UdpReceiver<T> {
    fn run(&self) {
        loop {
            let mut buffer = vec![0; T::max_packet_size()];
            let size = self.socket.recv(&mut buffer).unwrap();

            let data = T::deserialize_packet(&buffer[..size]);
            self.output.send(&data);
        }
    }
}
