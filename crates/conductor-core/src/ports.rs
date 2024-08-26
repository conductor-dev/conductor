use std::{
    cell::RefCell,
    rc::Rc,
    sync::mpsc::{channel, Receiver, RecvError, Sender},
};

pub struct InputPort<T> {
    tx: Sender<T>,
    rx: Receiver<T>,
}

impl<T> InputPort<T> {
    pub fn new() -> Self {
        let (tx, rx) = channel::<T>();
        Self { tx, rx }
    }

    pub fn recv(&self) -> Result<T, RecvError> {
        self.rx.recv()
    }
}

impl<T> Default for InputPort<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct InputPortCell<T>(Rc<RefCell<InputPort<T>>>);

impl<T> Clone for InputPortCell<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> From<InputPortCell<T>> for InputPort<T> {
    fn from(cell: InputPortCell<T>) -> Self {
        match Rc::try_unwrap(cell.0) {
            Ok(inner) => inner.into_inner(),
            Err(_) => unreachable!(),
        }
    }
}

impl<T> InputPortCell<T> {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(InputPort::new())))
    }

    pub fn recv(&self) -> Result<T, RecvError> {
        self.0.borrow().rx.recv()
    }
}

impl<T> Default for InputPortCell<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct OutputPort<T: Clone> {
    tx: Vec<Sender<T>>,
}

impl<T: Clone> OutputPort<T> {
    pub fn new() -> Self {
        Self { tx: Vec::new() }
    }

    pub fn connect(&mut self, input: &InputPortCell<T>) {
        self.tx.push(input.0.borrow().tx.clone());
    }

    pub fn send(&self, value: &T) {
        for tx in &self.tx {
            tx.send(value.clone()).unwrap();
        }
    }
}

impl<T: Clone> Default for OutputPort<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct OutputPortCell<T: Clone>(Rc<RefCell<OutputPort<T>>>);

impl<T: Clone> Clone for OutputPortCell<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: Clone> From<OutputPortCell<T>> for OutputPort<T> {
    fn from(cell: OutputPortCell<T>) -> Self {
        match Rc::try_unwrap(cell.0) {
            Ok(inner) => inner.into_inner(),
            Err(_) => unreachable!(),
        }
    }
}

impl<T: Clone> OutputPortCell<T> {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(OutputPort::new())))
    }

    pub fn connect(&self, input: &InputPortCell<T>) {
        self.0.borrow_mut().connect(input);
    }

    pub fn send(&self, value: &T) {
        self.0.borrow().send(value);
    }
}

impl<T: Clone> Default for OutputPortCell<T> {
    fn default() -> Self {
        Self::new()
    }
}
