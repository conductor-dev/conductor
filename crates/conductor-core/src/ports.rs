pub use crate::receive;
use crossbeam_channel::{Receiver, RecvError, Select, SelectedOperation, Sender, TryRecvError};
use std::{cell::RefCell, rc::Rc};

/*
receive! {
    self.input1 => msg => 0,
    self.input2 => msg => 1,
}
*/

#[macro_export]
macro_rules! receive {
    ($(($port:expr): $msg:ident => $output:expr),*) => {
        {
            let mut multi_receiver = $crate::ports::MultiReceiver::new();
            $(
                multi_receiver.recv(&$port);
            )*

            // let index = multi_receiver.select();

            // $(
            //     println!("{}: {}", 0, $output);
            // )*

            // match index {
            //     $(
            //         $msg => {
            //             let $msg = $port.recv().unwrap();
            //             $output
            //         },
            //     )*
            //     _ => unreachable!(),
            // }

            // TODO: This is really ugly. The counter could probably be removed by turning this into a recursive macro.
            loop {
                let oper = multi_receiver.select();
                let index = oper.index();
                let mut counter = 0;

                $(
                    if index == counter {
                        // let $msg = $port.recv().unwrap();
                        let $msg = oper.recv(&$port.rx).unwrap();
                        break $output;
                    }
                    counter += 1;
                )*

                unreachable!();
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

pub struct NodeRunnerInputPort<T> {
    tx: Sender<T>,
    pub rx: Receiver<T>, // TODO: make private
}

impl<T> NodeRunnerInputPort<T> {
    pub fn new() -> Self {
        let (tx, rx) = crossbeam_channel::unbounded::<T>();
        Self { tx, rx }
    }

    pub fn try_recv(&self) -> Result<T, TryRecvError> {
        self.rx.try_recv()
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
}

impl<T: Clone> Default for NodeConfigOutputPort<T> {
    fn default() -> Self {
        Self::new()
    }
}
