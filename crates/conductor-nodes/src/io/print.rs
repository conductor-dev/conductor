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

impl<T: Display> Node for ConsolePrinter<T> {
    // TODO: should be a trait method
    // TODO: does not need to be public
    fn run(&self) {
        loop {
            println!("{}", self.input.rx.recv().unwrap());
        }
    }
}
