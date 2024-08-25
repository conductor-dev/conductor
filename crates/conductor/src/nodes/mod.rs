use std::sync::mpsc::{channel, Receiver, Sender};

pub mod print;
pub mod udp;

/// SinkPort / InputPort
pub struct SinkPort<T> {
    tx: Sender<T>,
    rx: Receiver<T>,
}

impl<T> SinkPort<T> {
    fn new() -> Self {
        let (tx, rx) = channel::<T>();
        Self { tx, rx }
    }
}

/// SourcePort / OutputPort
pub struct SourcePort<T: Clone> {
    tx: Vec<Sender<T>>,
}

impl<T: Clone> SourcePort<T> {
    fn new() -> Self {
        Self { tx: Vec::new() }
    }

    pub fn connect(&mut self, input: &SinkPort<T>) {
        self.tx.push(input.tx.clone());
    }

    fn send(&self, value: &T) {
        for tx in &self.tx {
            tx.send(value.clone()).unwrap();
        }
    }
}

pub trait Node {
    fn run(&self);
}
