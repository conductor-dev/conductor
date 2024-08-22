use crate::{
    node::Node,
    traits::{CreateRunner, Runner, TransmitNode},
};
use std::sync::mpsc::{Receiver, Sender};

#[derive(Debug)]
pub(crate) struct PassRunner {
    receive_channel: Receiver<f32>,
    send_channels: Vec<Sender<f32>>,
}

impl Runner for PassRunner {
    fn run(&self) {
        loop {
            let message = self.receive_channel.recv().unwrap();

            for channel in &self.send_channels {
                channel.send(message).unwrap();
            }
        }
    }

    fn add_output_channel(&mut self, channel: Sender<f32>) {
        self.send_channels.push(channel);
    }
}

#[derive(Debug)]
pub struct Pass {}

impl<'a> From<&'a Pass> for Node<'a> {
    fn from(value: &'a Pass) -> Self {
        Self::TransmitNode(value)
    }
}

impl TransmitNode for Pass {}

impl CreateRunner for Pass {
    fn create_runner(
        &self,
        send_channels: Vec<Sender<f32>>,
        receive_channel: Receiver<f32>,
    ) -> Box<dyn Runner> {
        Box::new(PassRunner {
            receive_channel,
            send_channels,
        })
    }
}

impl Pass {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Pass {
    fn default() -> Self {
        Self::new()
    }
}
