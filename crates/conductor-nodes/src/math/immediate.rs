use conductor_core::{
    ports::{NodeConfigOutputPort, NodeRunnerOutputPort},
    NodeConfig, NodeRunner,
};

struct ImmediateRunner<T: Clone> {
    value: T,
    output: NodeRunnerOutputPort<T>,
}

impl<T: Clone> NodeRunner for ImmediateRunner<T> {
    fn run(self: Box<Self>) {
        self.output.send(&self.value);
    }
}

pub struct Immediate<T: Clone> {
    value: T,
    pub output: NodeConfigOutputPort<T>,
}

impl<T: Clone> Immediate<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            output: NodeConfigOutputPort::<T>::new(),
        }
    }
}

impl<T: Clone + Send + 'static> NodeConfig for Immediate<T> {
    fn into_runner(self: Box<Self>) -> Box<dyn NodeRunner + Send> {
        Box::new(ImmediateRunner {
            value: self.value,
            output: self.output.into(),
        })
    }
}
