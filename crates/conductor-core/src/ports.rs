pub use crate::receive;
use crossbeam_channel::{Receiver, Select, SelectedOperation, Sender, TryRecvError};
use std::sync::{Arc, RwLock};

/// ```ignore
/// use conductor::prelude::*;
///
/// let input1 = NodeRunnerInputPort::<f32>::new();
/// let input2 = NodeRunnerInputPort::<f32>::new();
///
/// receive! {
///     (input1): msg => println!("Received input1: {:?}", msg),
///     (input2): msg => println!("Received input2: {:?}", msg),
/// };
/// ```
#[macro_export]
macro_rules! receive {
    ($(($port:expr): $msg:ident => $output:expr),* $(,)?) => {
        {
            let mut multi_receiver = $crate::ports::MultiReceiver::new();
            $(
                if let $crate::ports::PortKind::Eager = $port.kind() {
                    multi_receiver.recv(&$port);
                }
            )*

            // TODO: This is really ugly. The counter could probably be removed by turning this into a recursive macro.
            loop {
                let oper = multi_receiver.select();
                let index = oper.index();
                let mut counter = 0;

                $(
                    if let $crate::ports::PortKind::LazyDrop = $port.kind() {
                        if let Ok($msg) = $port.try_recv_last() {
                           $output;
                        }
                    }
                    if let $crate::ports::PortKind::LazyBuffer = $port.kind() {
                        if let Ok($msg) = $port.try_recv() {
                           $output;
                        }
                    }
                )*
                $(
                    if let $crate::ports::PortKind::Eager = $port.kind() {
                        if index == counter {
                            let $msg = $port.recv_select(oper);
                            $output;
                            break;
                        }
                        counter += 1;
                    }
                )*
            }
        }
    };
}

pub struct MultiReceiver<'a> {
    select: Select<'a>,
}

impl<'a> MultiReceiver<'a> {
    pub fn new() -> Self {
        Self {
            select: Select::new(),
        }
    }

    pub fn recv<T>(&mut self, port: &'a NodeRunnerInputPort<T>) {
        self.select.recv(&port.rx);
    }

    pub fn select(&mut self) -> SelectedOperation<'a> {
        self.select.select()
    }
}

impl Default for MultiReceiver<'_> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy)]
pub enum PortKind {
    Eager,
    LazyDrop,
    LazyBuffer,
}

pub struct NodeRunnerInputPort<T> {
    tx: Sender<T>,
    rx: Receiver<T>,
    kind: PortKind,
}

impl<T> NodeRunnerInputPort<T> {
    pub fn new() -> Self {
        let (tx, rx) = crossbeam_channel::unbounded::<T>();
        Self {
            tx,
            rx,
            kind: PortKind::Eager,
        }
    }

    pub fn try_recv_last(&self) -> Result<T, TryRecvError> {
        let mut last = None;
        while let Ok(value) = self.try_recv() {
            last = Some(value);
        }
        last.ok_or(TryRecvError::Empty)
    }

    pub fn try_recv(&self) -> Result<T, TryRecvError> {
        self.rx.try_recv()
    }

    pub fn recv(&self) -> T {
        if let Ok(message) = self.rx.recv() {
            message
        } else {
            eprintln!("ERROR: Tried to receive from disconnected port. Node may have panicked.");
            loop {
                std::thread::park();
            }
        }
    }

    pub fn recv_select(&self, select: SelectedOperation) -> T {
        if let Ok(message) = select.recv(&self.rx) {
            message
        } else {
            loop {
                std::thread::park();
            }
        }
    }

    pub fn kind(&self) -> PortKind {
        self.kind
    }
}

impl<T> Default for NodeRunnerInputPort<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct NodeConfigInputPort<T>(Arc<RwLock<NodeRunnerInputPort<T>>>);

impl<T> Clone for NodeConfigInputPort<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> From<NodeConfigInputPort<T>> for NodeRunnerInputPort<T> {
    fn from(cell: NodeConfigInputPort<T>) -> Self {
        match Arc::try_unwrap(cell.0) {
            Ok(inner) => inner.into_inner().expect("poisoned lock"),
            Err(_) => unreachable!(),
        }
    }
}

impl<T> NodeConfigInputPort<T> {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(NodeRunnerInputPort::new())))
    }

    pub fn set_initial(&self, value: T) {
        self.0
            .write()
            .expect("poisoned lock")
            .tx
            .send(value)
            .unwrap();
    }

    pub fn set_kind(&self, kind: PortKind) {
        self.0.write().expect("poisoned lock").kind = kind;
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
            if tx.send(value.clone()).is_err() {
                eprintln!("ERROR: Tried to send to disconnected port. Node may have panicked.");
            }
        }
    }
}

impl<T: Clone> Default for NodeRunnerOutputPort<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct NodeConfigOutputPort<T: Clone>(Arc<RwLock<NodeRunnerOutputPort<T>>>);

impl<T: Clone> Clone for NodeConfigOutputPort<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: Clone> From<NodeConfigOutputPort<T>> for NodeRunnerOutputPort<T> {
    fn from(cell: NodeConfigOutputPort<T>) -> Self {
        match Arc::try_unwrap(cell.0) {
            Ok(inner) => inner.into_inner().expect("poisoned lock"),
            Err(_) => unreachable!(),
        }
    }
}

impl<T: Clone> NodeConfigOutputPort<T> {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(NodeRunnerOutputPort::new())))
    }

    pub fn connect(&self, input: &NodeConfigInputPort<T>) {
        self.0
            .write()
            .expect("poisoned lock")
            .tx
            .push(input.0.read().expect("poisoned lock").tx.clone());
    }
}

impl<T: Clone> Default for NodeConfigOutputPort<T> {
    fn default() -> Self {
        Self::new()
    }
}
