use conductor_core::{
    node::{Node, SinkPort, SinkPortCell},
    runner::Runner,
};
use std::fmt::Display;

pub struct ConsolePrinterRunner<T: Display> {
    pub input: SinkPort<T>,
}

impl<T: Display> Runner for ConsolePrinterRunner<T> {
    fn run(&self) {
        loop {
            println!("{}", self.input.recv().unwrap());
        }
    }
}

pub struct ConsolePrinter<T: Display> {
    pub input: SinkPortCell<T>,
}

impl<T: Display> ConsolePrinter<T> {
    pub fn new() -> Self {
        Self {
            input: SinkPortCell::<T>::new(),
        }
    }
}

impl Default for ConsolePrinter<f32> {
    fn default() -> Self {
        Self::new()
    }
}

// TODO: Can + Send + 'static be removed?
impl<T: Display + Send + 'static> Node for ConsolePrinter<T> {
    fn create_runner(self: Box<Self>) -> Box<dyn Runner + Send> {
        Box::new(ConsolePrinterRunner {
            input: self.input.into(),
        })
    }
}
