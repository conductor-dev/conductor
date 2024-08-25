use std::fmt::Display;

use conductor_core::nodes::{Node, SinkPort};

pub struct Pass<T> {
    pub input: SinkPort<T>,
    pub output: SinkPort<T>,
}

impl<T: Display> Pass<T> {
    pub fn new() -> Self {
        Self {
            input: SinkPort::<T>::new(),
            output: SinkPort::<T>::new(),
        }
    }
}

impl<T: Display> Node for Pass<T> {
    // TODO: should be a trait method
    // TODO: does not need to be public
    fn run(&self) {
        loop {
            let value = self.input.rx.recv().unwrap();
            self.output.tx.send(value).unwrap();
        }
    }
}
