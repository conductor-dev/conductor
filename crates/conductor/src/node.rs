use crate::{
    orchestrator::Pipeline,
    traits::{InputNode, OutputNode, TransmitNode},
};

#[derive(Debug, Clone)]
pub enum Node<'a> {
    InputNode(&'a dyn InputNode),
    OutputNode(&'a dyn OutputNode),
    TransmitNode(&'a dyn TransmitNode),
    Pipeline(&'a Pipeline<'a>),
}

impl<'a> Node<'a> {
    pub(crate) fn get_address(&self) -> usize {
        match self {
            Node::InputNode(node) => *node as *const dyn InputNode as *const () as usize,
            Node::OutputNode(node) => *node as *const dyn OutputNode as *const () as usize,
            Node::TransmitNode(node) => *node as *const dyn TransmitNode as *const () as usize,
            Node::Pipeline(pipeline) => *pipeline as *const Pipeline as *const () as usize,
        }
    }
}
