use crate::nodes::{Node, SinkPort};
use std::fmt::Display;

pub struct ConsolePrinterNode<T: Display> {
    pub input: SinkPort<T>,
}

impl<T: Display> ConsolePrinterNode<T> {
    pub fn new() -> Self {
        Self {
            input: SinkPort::<T>::new(),
        }
    }
}

impl<T: Display> Node for ConsolePrinterNode<T> {
    // TODO: should be a trait method
    // TODO: does not need to be public
    fn run(&self) {
        loop {
            println!("{}", self.input.rx.recv().unwrap());
        }
    }
}
