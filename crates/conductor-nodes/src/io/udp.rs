use conductor_core::{
    ports::{OutputPort, OutputPortCell},
    Node, Runner,
};
use std::net::UdpSocket;

pub trait UdpDeserializer {
    fn max_packet_size() -> usize;
    fn deserialize_packet(bytes: &[u8]) -> Self;
}

pub struct UdpReceiverRunner<T: Clone + UdpDeserializer> {
    socket: UdpSocket,
    output: OutputPort<T>,
}

impl<T: Clone + UdpDeserializer> Runner for UdpReceiverRunner<T> {
    fn run(self: Box<Self>) {
        loop {
            let mut buffer = vec![0; T::max_packet_size()];
            let size = self.socket.recv(&mut buffer).unwrap();

            let data = T::deserialize_packet(&buffer[..size]);
            self.output.send(&data);
        }
    }
}

pub struct UdpReceiver<'a, T: Clone + UdpDeserializer> {
    addr: &'a str,
    pub output: OutputPortCell<T>,
}

impl<'a, T: Clone + UdpDeserializer> UdpReceiver<'a, T> {
    pub fn new(addr: &'a str) -> Self {
        Self {
            addr,
            output: OutputPortCell::<T>::new(),
        }
    }
}

// TODO: Can + Send + 'static be removed?
impl<'a, T: Clone + UdpDeserializer + Send + 'static> Node for UdpReceiver<'a, T> {
    fn create_runner(self: Box<Self>) -> Box<dyn Runner + Send> {
        let socket = UdpSocket::bind(self.addr).unwrap();

        Box::new(UdpReceiverRunner {
            socket,
            output: self.output.into(),
        })
    }
}
