use std::{
    cell::RefCell,
    rc::Rc,
    sync::mpsc::{channel, Receiver, RecvError, Sender},
};

pub struct NodeRunnerInputPort<T> {
    tx: Sender<T>,
    rx: Receiver<T>,
}

impl<T> NodeRunnerInputPort<T> {
    pub fn new() -> Self {
        let (tx, rx) = channel::<T>();
        Self { tx, rx }
    }

    pub fn recv(&self) -> Result<T, RecvError> {
        self.rx.recv()
    }
}

impl<T> Default for NodeRunnerInputPort<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct NodeConfigInputPort<T>(Rc<RefCell<NodeRunnerInputPort<T>>>);

impl<T> Clone for NodeConfigInputPort<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> From<NodeConfigInputPort<T>> for NodeRunnerInputPort<T> {
    fn from(cell: NodeConfigInputPort<T>) -> Self {
        match Rc::try_unwrap(cell.0) {
            Ok(inner) => inner.into_inner(),
            Err(_) => unreachable!(),
        }
    }
}

impl<T> NodeConfigInputPort<T> {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(NodeRunnerInputPort::new())))
    }

    pub fn recv(&self) -> Result<T, RecvError> {
        self.0.borrow().rx.recv()
    }
}

impl<T> Default for NodeConfigInputPort<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct NodeRunnerOutputPort<T: Clone> {
    tx: Vec<Sender<T>>,
}

impl<T: Clone> NodeRunnerOutputPort<T> {
    pub fn new() -> Self {
        Self { tx: Vec::new() }
    }

    pub fn send(&self, value: &T) {
        for tx in &self.tx {
            tx.send(value.clone()).unwrap();
        }
    }
}

impl<T: Clone> Default for NodeRunnerOutputPort<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct NodeConfigOutputPort<T: Clone>(Rc<RefCell<NodeRunnerOutputPort<T>>>);

impl<T: Clone> Clone for NodeConfigOutputPort<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: Clone> From<NodeConfigOutputPort<T>> for NodeRunnerOutputPort<T> {
    fn from(cell: NodeConfigOutputPort<T>) -> Self {
        match Rc::try_unwrap(cell.0) {
            Ok(inner) => inner.into_inner(),
            Err(_) => unreachable!(),
        }
    }
}

impl<T: Clone> NodeConfigOutputPort<T> {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(NodeRunnerOutputPort::new())))
    }

    pub fn connect(&self, input: &NodeConfigInputPort<T>) {
        self.0.borrow_mut().tx.push(input.0.borrow().tx.clone());
    }

    pub fn send(&self, value: &T) {
        self.0.borrow().send(value);
    }
}

impl<T: Clone> Default for NodeConfigOutputPort<T> {
    fn default() -> Self {
        Self::new()
    }
}
