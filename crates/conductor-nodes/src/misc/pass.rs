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
    fn run(&self) {
        loop {
            let value = self.input.rx.recv().unwrap();
            self.output.tx.send(value).unwrap();
        }
    }
}

impl<T: Display> Default for Pass<T> {
    fn default() -> Self {
        Self::new()
    }
}
