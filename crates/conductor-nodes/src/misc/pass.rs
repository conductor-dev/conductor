use conductor_core::{
    ports::{NodeConfigInputPort, NodeConfigOutputPort, NodeRunnerInputPort, NodeRunnerOutputPort},
    NodeConfig, NodeRunner,
};

struct PassRunner<T: Clone> {
    input: NodeRunnerInputPort<T>,
    output: NodeRunnerOutputPort<T>,
}

impl<T: Clone> NodeRunner for PassRunner<T> {
    fn run(self: Box<Self>) {
        loop {
            let value = self.input.recv().unwrap();
            self.output.send(&value);
        }
    }
}

pub struct Pass<T: Clone> {
    pub input: NodeConfigInputPort<T>,
    pub output: NodeConfigOutputPort<T>,
}

impl<T: Clone> Pass<T> {
    pub fn new() -> Self {
        Self {
            input: NodeConfigInputPort::<T>::new(),
            output: NodeConfigOutputPort::<T>::new(),
        }
    }
}

impl<T: Clone> Default for Pass<T> {
    fn default() -> Self {
        Self::new()
    }
}

// TODO: Can + Send + 'static be removed?
impl<T: Clone + Send + 'static> NodeConfig for Pass<T> {
    fn into_runner(self: Box<Self>) -> Box<dyn NodeRunner + Send> {
        Box::new(PassRunner {
            input: self.input.into(),
            output: self.output.into(),
        })
    }
}
