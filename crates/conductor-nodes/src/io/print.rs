use conductor_core::{
    ports::{NodeConfigInputPort, NodeRunnerInputPort},
    NodeConfig, NodeRunner,
};
use std::fmt::Display;

struct ConsolePrinterRunner<T: Display> {
    input: NodeRunnerInputPort<T>,
}

impl<T: Display> NodeRunner for ConsolePrinterRunner<T> {
    fn run(self: Box<Self>) {
        loop {
            println!("{}", self.input.recv());
        }
    }
}

pub struct ConsolePrinter<T: Display> {
    pub input: NodeConfigInputPort<T>,
}

impl<T: Display> ConsolePrinter<T> {
    pub fn new() -> Self {
        Self {
            input: NodeConfigInputPort::<T>::new(),
        }
    }
}

impl<T: Display> Default for ConsolePrinter<T> {
    fn default() -> Self {
        Self::new()
    }
}

// TODO: Can + Send + 'static be removed?
impl<T: Display + Send + 'static> NodeConfig for ConsolePrinter<T> {
    fn into_runner(self: Box<Self>) -> Box<dyn NodeRunner + Send> {
        Box::new(ConsolePrinterRunner {
            input: self.input.into(),
        })
    }
}
