use conductor_core::{
    ports::{NodeConfigOutputPort, NodeRunnerOutputPort},
    NodeConfig, NodeRunner,
};
use std::net::UdpSocket;

pub trait UdpDeserializer {
    fn max_packet_size() -> usize;
    fn deserialize_packet(bytes: &[u8]) -> Self;
}

struct UdpReceiverRunner<T: Clone + UdpDeserializer> {
    socket: UdpSocket,
    output: NodeRunnerOutputPort<T>,
}

impl<T: Clone + UdpDeserializer> NodeRunner for UdpReceiverRunner<T> {
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
    pub output: NodeConfigOutputPort<T>,
}

impl<'a, T: Clone + UdpDeserializer> UdpReceiver<'a, T> {
    pub fn new(addr: &'a str) -> Self {
        Self {
            addr,
            output: NodeConfigOutputPort::<T>::new(),
        }
    }
}

// TODO: Can + Send + 'static be removed?
impl<'a, T: Clone + UdpDeserializer + Send + 'static> NodeConfig for UdpReceiver<'a, T> {
    fn into_runner(self: Box<Self>) -> Box<dyn NodeRunner + Send> {
        let socket = UdpSocket::bind(self.addr).unwrap();

        Box::new(UdpReceiverRunner {
            socket,
            output: self.output.into(),
        })
    }
}
