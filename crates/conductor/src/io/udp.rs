use crate::{
    node::Node,
    traits::{CreateRunner, InputNode, Runner, SendTo},
};
use std::{
    net::SocketAddr,
    sync::mpsc::{Receiver, Sender},
};

#[derive(Debug)]
pub(crate) struct UdpSocketRunner {
    socket: std::net::UdpSocket,
    output_channels: Vec<Sender<f32>>,
}

impl Runner for UdpSocketRunner {
    fn run(&self) {
        loop {
            let mut buffer = [0; 4]; // 4 bytes for f32

            self.socket.recv_from(&mut buffer).unwrap();

            let message = f32::from_ne_bytes(buffer);

            for channel in &self.output_channels {
                channel.send(message).unwrap();
            }
        }
    }

    fn add_output_channel(&mut self, channel: Sender<f32>) {
        self.output_channels.push(channel);
    }
}

impl UdpSocketRunner {
    pub fn new(address: SocketAddr, output_channels: Vec<Sender<f32>>) -> Self {
        Self {
            socket: std::net::UdpSocket::bind(address).unwrap(),
            output_channels,
        }
    }
}

#[derive(Debug)]
pub struct UdpSocket {
    address: SocketAddr,
}

impl CreateRunner for UdpSocket {
    fn create_runner(
        &self,
        send_channels: Vec<Sender<f32>>,
        _receive_channel: Receiver<f32>,
    ) -> Box<dyn Runner> {
        Box::new(UdpSocketRunner::new(self.address, send_channels))
    }
}

impl InputNode for UdpSocket {}

impl<'a> From<&'a UdpSocket> for Node<'a> {
    fn from(value: &'a UdpSocket) -> Self {
        Node::InputNode(value)
    }
}

impl SendTo for UdpSocket {}

impl UdpSocket {
    pub fn new(address: SocketAddr) -> Self {
        Self { address }
    }
}
