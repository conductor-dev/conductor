use crate::{
    node::Node,
    traits::{CreateRunner, OutputNode, Runner},
};
use std::sync::mpsc::{Receiver, Sender};

#[derive(Debug)]
pub(crate) struct PrintOutputRunner {
    receive_channel: Receiver<f32>,
}

impl Runner for PrintOutputRunner {
    fn run(&self) {
        loop {
            let message = self.receive_channel.recv().unwrap();

            println!("{}", message);
        }
    }

    fn add_output_channel(&mut self, _channel: Sender<f32>) {}
}

#[derive(Debug)]
pub struct PrintOutput {}

impl<'a> From<&'a PrintOutput> for Node<'a> {
    fn from(value: &'a PrintOutput) -> Self {
        Self::OutputNode(value)
    }
}

impl CreateRunner for PrintOutput {
    fn create_runner(
        &self,
        _send_channels: Vec<Sender<f32>>,
        receive_channel: Receiver<f32>,
    ) -> Box<dyn Runner> {
        Box::new(PrintOutputRunner { receive_channel })
    }
}

impl OutputNode for PrintOutput {}

impl PrintOutput {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for PrintOutput {
    fn default() -> Self {
        Self::new()
    }
}
