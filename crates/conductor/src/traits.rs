use crate::{node::Node, orchestrator::Pipeline};
use std::{
    fmt::Debug,
    sync::mpsc::{Receiver, Sender},
};

pub trait Runner: Send + Debug {
    fn run(&self);

    fn add_output_channel(&mut self, channel: Sender<f32>);
}

pub trait CreateRunner {
    fn create_runner(
        &self,
        send_channels: Vec<Sender<f32>>,
        receive_channel: Receiver<f32>,
    ) -> Box<dyn Runner>;
}

pub trait SendTo {
    fn send_to<'a, N>(&'a self, node: N) -> Pipeline<'a>
    where
        N: Into<Node<'a>>,
        Node<'a>: From<&'a Self>,
    {
        let pipeline: Pipeline = Node::from(self).into();

        pipeline.send_to(node)
    }
}

pub trait InputNode: CreateRunner + Debug {}

pub trait OutputNode: CreateRunner + Debug {}

pub trait TransmitNode: CreateRunner + Debug {}
