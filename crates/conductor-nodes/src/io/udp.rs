use conductor_core::{
    ports::{NodeConfigInputPort, NodeConfigOutputPort, NodeRunnerInputPort, NodeRunnerOutputPort},
    NodeConfig, NodeRunner,
};
use std::net::UdpSocket;

pub trait UdpDeserializer {
    fn max_packet_size() -> usize;
    fn deserialize_packet(bytes: &[u8]) -> Self;
}

pub trait UdpSerializer {
    fn serialize_packet(self) -> Vec<u8>;
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

struct UdpSenderRunner<T: UdpSerializer> {
    socket: UdpSocket,
    send_addr: String,
    input: NodeRunnerInputPort<T>,
}

impl<T: UdpSerializer> NodeRunner for UdpSenderRunner<T> {
    fn run(self: Box<Self>) {
        loop {
            let data = self.input.recv().unwrap();

            let buffer = data.serialize_packet();

            self.socket.send_to(&buffer, &self.send_addr).unwrap();
        }
    }
}

pub struct UdpReceiver<'a, T: Clone + UdpDeserializer> {
    bind_addr: &'a str,
    pub output: NodeConfigOutputPort<T>,
}

impl<'a, T: Clone + UdpDeserializer> UdpReceiver<'a, T> {
    pub fn new(bind_addr: &'a str) -> Self {
        Self {
            bind_addr,
            output: NodeConfigOutputPort::<T>::new(),
        }
    }
}

// TODO: Can + Send + 'static be removed?
impl<'a, T: Clone + UdpDeserializer + Send + 'static> NodeConfig for UdpReceiver<'a, T> {
    fn into_runner(self: Box<Self>) -> Box<dyn NodeRunner + Send> {
        let socket = UdpSocket::bind(self.bind_addr).unwrap();

        Box::new(UdpReceiverRunner {
            socket,
            output: self.output.into(),
        })
    }
}

pub struct UdpSender<T: UdpSerializer> {
    bind_addr: String,
    send_addr: String,
    pub input: NodeConfigInputPort<T>,
}

impl<T: UdpSerializer> UdpSender<T> {
    pub fn new<A: Into<String>, B: Into<String>>(bind_addr: A, send_addr: B) -> Self {
        Self {
            bind_addr: bind_addr.into(),
            send_addr: send_addr.into(),
            input: NodeConfigInputPort::new(),
        }
    }
}

impl<T: UdpSerializer + Send + 'static> NodeConfig for UdpSender<T> {
    fn into_runner(self: Box<Self>) -> Box<dyn NodeRunner + Send> {
        let socket = UdpSocket::bind(self.bind_addr).unwrap();

        Box::new(UdpSenderRunner {
            socket,
            send_addr: self.send_addr.to_string(),
            input: self.input.into(),
        })
    }
}
