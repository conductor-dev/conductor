use conductor_core::{
    ports::{NodeConfigInputPort, NodeConfigOutputPort, NodeRunnerInputPort, NodeRunnerOutputPort},
    NodeConfig, NodeRunner,
};

struct IntoRunner<I, O: From<I> + Clone> {
    input: NodeRunnerInputPort<I>,
    output: NodeRunnerOutputPort<O>,
}

impl<I, O: From<I> + Clone> NodeRunner for IntoRunner<I, O> {
    fn run(self: Box<Self>) {
        loop {
            let value = self.input.recv().unwrap();
            self.output.send(&value.into());
        }
    }
}

pub struct IntoNode<I, O: From<I> + Clone> {
    pub input: NodeConfigInputPort<I>,
    pub output: NodeConfigOutputPort<O>,
}

impl<I, O: From<I> + Clone> IntoNode<I, O> {
    pub fn new() -> Self {
        Self {
            input: NodeConfigInputPort::new(),
            output: NodeConfigOutputPort::new(),
        }
    }
}

impl<I, O: From<I> + Clone> Default for IntoNode<I, O> {
    fn default() -> Self {
        Self::new()
    }
}

// TODO: Can + Send + 'static be removed?
impl<I: Send + 'static, O: From<I> + Clone + Send + 'static> NodeConfig for IntoNode<I, O> {
    fn into_runner(self: Box<Self>) -> Box<dyn NodeRunner + Send> {
        Box::new(IntoRunner {
            input: self.input.into(),
            output: self.output.into(),
        })
    }
}
