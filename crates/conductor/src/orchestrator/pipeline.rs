use std::{
    collections::{hash_map::Entry, HashMap},
    sync::mpsc::Sender,
};

use crate::{node::Node, traits::Runner};

#[derive(Debug, Clone)]
pub struct Pipeline<'a> {
    pub(crate) nodes: Vec<Node<'a>>,
}

impl<'a> From<Node<'a>> for Pipeline<'a> {
    fn from(value: Node<'a>) -> Self {
        Self { nodes: vec![value] }
    }
}

impl<'a> From<&'a Pipeline<'a>> for Node<'a> {
    fn from(value: &'a Pipeline<'a>) -> Self {
        Node::Pipeline(value)
    }
}

impl<'a> Pipeline<'a> {
    pub fn send_to<N>(self, node: N) -> Self
    where
        N: Into<Node<'a>>,
    {
        let mut nodes = self.nodes.clone();

        nodes.push(node.into());

        Pipeline { nodes }
    }

    pub fn evaluate(
        &self,
        mapping: &mut HashMap<usize, Sender<f32>>,
        runners: &mut HashMap<usize, Box<dyn Runner>>,
        last_node: Option<&Node<'a>>,
    ) {
        let mut last_node = last_node;

        for node in self.nodes.iter().rev() {
            if let Node::Pipeline(pipeline) = node {
                // TODO: handle communication between pipelines
                pipeline.evaluate(mapping, runners, last_node);

                continue;
            }

            let ptr = node.get_address();

            let last_node_sender = last_node.map(|last_node| {
                let ptr = last_node.get_address();

                mapping.get(&ptr).expect("Must exist").clone()
            });
            let last_node_runner = last_node.map(|last_node| {
                let ptr = last_node.get_address();

                runners.get_mut(&ptr).expect("Must exist")
            });

            match mapping.entry(ptr) {
                Entry::Occupied(entry) => match node {
                    Node::InputNode(_) => {
                        if last_node.is_none() {
                            panic!("Input node must be the first node in the pipeline");
                        }

                        let runner = runners.get_mut(&ptr).expect("must exist");

                        runner.add_output_channel(last_node_sender.expect("must exist"));
                    }
                    Node::OutputNode(_) => {
                        if last_node.is_some() {
                            panic!("Output node must be the last node in the pipeline");
                        }
                    }
                    Node::TransmitNode(_) => {
                        if last_node.is_none() {
                            panic!("Transmit node must not be the last node in the pipeline");
                        }

                        last_node_runner
                            .expect("must exist")
                            .add_output_channel(entry.get().clone());

                        let runner = runners.get_mut(&ptr).expect("must exist");

                        runner.add_output_channel(last_node_sender.expect("must exist"));
                    }
                    _ => {}
                },
                Entry::Vacant(entry) => {
                    let (send, receive) = std::sync::mpsc::channel();

                    let runner = match node {
                        Node::InputNode(node) => node.create_runner(
                            {
                                if last_node.is_none() {
                                    panic!("Input node must be the first node in the pipeline");
                                }

                                vec![last_node_sender.expect("must exist")]
                            },
                            receive,
                        ),
                        Node::OutputNode(node) => node.create_runner(
                            {
                                if last_node.is_some() {
                                    panic!("Output node must be the last node in the pipeline");
                                }

                                vec![]
                            },
                            receive,
                        ),
                        Node::TransmitNode(node) => node.create_runner(
                            {
                                if last_node.is_none() {
                                    panic!(
                                        "Transmit node must not be the last node in the pipeline"
                                    );
                                }

                                last_node_runner
                                    .expect("must exist")
                                    .add_output_channel(send.clone());

                                vec![last_node_sender.expect("must exist")]
                            },
                            receive,
                        ),
                        _ => unreachable!(),
                    };

                    runners.insert(ptr, runner);

                    entry.insert(send);
                }
            }

            last_node = Some(node);
        }
    }
}
