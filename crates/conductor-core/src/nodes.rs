use std::sync::mpsc::{channel, Receiver, Sender};

pub struct SinkPort<T> {
    // TODO: hide Receiver and Sender from public API?
    pub tx: Sender<T>,
    pub rx: Receiver<T>,
}

impl<T> SinkPort<T> {
    pub fn new() -> Self {
        let (tx, rx) = channel::<T>();
        Self { tx, rx }
    }
}

pub struct SourcePort<T: Clone> {
    tx: Vec<Sender<T>>,
}

impl<T: Clone> SourcePort<T> {
    pub fn new() -> Self {
        Self { tx: Vec::new() }
    }

    pub fn connect(&mut self, input: &SinkPort<T>) {
        self.tx.push(input.tx.clone());
    }

    pub fn send(&self, value: &T) {
        for tx in &self.tx {
            tx.send(value.clone()).unwrap();
        }
    }
}

pub trait Node {
    fn run(&self);
}
