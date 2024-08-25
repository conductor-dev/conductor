use conductor_core::nodes::{Node, SourcePort};
use std::net::UdpSocket;

pub trait DeserializeFromBytes {
    fn deserialize_from_bytes(bytes: &[u8]) -> Self;
}

pub struct UdpReceiver<T: Clone + DeserializeFromBytes> {
    socket: UdpSocket,
    pub output: SourcePort<T>,
}

impl<T: Clone + DeserializeFromBytes> UdpReceiver<T> {
    pub fn new(addr: &str) -> Self {
        let socket = UdpSocket::bind(addr).unwrap();

        Self {
            socket,
            output: SourcePort::<T>::new(),
        }
    }
}

impl<T: Clone + DeserializeFromBytes> Node for UdpReceiver<T> {
    // TODO: should be a trait method
    // TODO: does not need to be public
    fn run(&self) {
        loop {
            // TODO: this is not ideal
            let len = self.socket.peek(&mut [0; 100]).unwrap();
            let mut buffer = vec![0; len];

            self.socket.recv_from(&mut buffer).unwrap();
            self.output.send(&T::deserialize_from_bytes(&buffer));
        }
    }
}
