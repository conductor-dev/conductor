use conductor_core::nodes::{Node, SinkPort};
use std::fmt::Display;

pub struct ConsolePrinter<T: Display> {
    pub input: SinkPort<T>,
}

impl<T: Display> ConsolePrinter<T> {
    pub fn new() -> Self {
        Self {
            input: SinkPort::<T>::new(),
        }
    }
}

impl Default for ConsolePrinter<f32> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Display> Node for ConsolePrinter<T> {
    fn run(&self) {
        loop {
            println!("{}", self.input.rx.recv().unwrap());
        }
    }
}
