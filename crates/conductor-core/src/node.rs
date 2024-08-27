use crate::runner::Runner;
use std::{
    cell::RefCell,
    rc::Rc,
    sync::mpsc::{channel, Receiver, RecvError, Sender},
};

pub struct SinkPort<T> {
    tx: Sender<T>,
    rx: Receiver<T>,
}

impl<T> SinkPort<T> {
    pub fn new() -> Self {
        let (tx, rx) = channel::<T>();
        Self { tx, rx }
    }

    pub fn recv(&self) -> Result<T, RecvError> {
        self.rx.recv()
    }
}

impl<T> Default for SinkPort<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SinkPortCell<T>(Rc<RefCell<SinkPort<T>>>);

impl<T> Clone for SinkPortCell<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> From<SinkPortCell<T>> for SinkPort<T> {
    fn from(cell: SinkPortCell<T>) -> Self {
        match Rc::try_unwrap(cell.0) {
            Ok(inner) => inner.into_inner(),
            Err(_) => unreachable!(),
        }
    }
}

impl<T> SinkPortCell<T> {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(SinkPort::new())))
    }

    pub fn recv(&self) -> Result<T, RecvError> {
        self.0.borrow().rx.recv()
    }
}

impl<T> Default for SinkPortCell<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SourcePort<T: Clone> {
    tx: Vec<Sender<T>>,
}

impl<T: Clone> Clone for SourcePortCell<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: Clone> SourcePort<T> {
    pub fn new() -> Self {
        Self { tx: Vec::new() }
    }

    pub fn connect(&mut self, input: &SinkPortCell<T>) {
        self.tx.push(input.0.borrow().tx.clone());
    }

    pub fn send(&self, value: &T) {
        for tx in &self.tx {
            tx.send(value.clone()).unwrap();
        }
    }
}

impl<T: Clone> Default for SourcePort<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SourcePortCell<T: Clone>(Rc<RefCell<SourcePort<T>>>);

impl<T: Clone> From<SourcePortCell<T>> for SourcePort<T> {
    fn from(cell: SourcePortCell<T>) -> Self {
        match Rc::try_unwrap(cell.0) {
            Ok(inner) => inner.into_inner(),
            Err(_) => unreachable!(),
        }
    }
}

impl<T: Clone> SourcePortCell<T> {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(SourcePort::new())))
    }

    pub fn connect(&self, input: &SinkPortCell<T>) {
        self.0.borrow_mut().connect(input);
    }

    pub fn send(&self, value: &T) {
        self.0.borrow().send(value);
    }
}

impl<T: Clone> Default for SourcePortCell<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Node {
    fn create_runner(self: Box<Self>) -> Box<dyn Runner + Send>;
}
