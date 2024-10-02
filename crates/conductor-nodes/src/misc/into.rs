use conductor_core::{
    ports::{NodeConfigInputPort, NodeConfigOutputPort, NodeRunnerInputPort, NodeRunnerOutputPort},
    NodeConfig, NodeRunner,
};

struct IntoerRunner<I, O: From<I> + Clone> {
    input: NodeRunnerInputPort<I>,
    output: NodeRunnerOutputPort<O>,
}

impl<I, O: From<I> + Clone> NodeRunner for IntoerRunner<I, O> {
    fn run(self: Box<Self>) {
        loop {
            let value = self.input.recv().unwrap();
            self.output.send(&value.into());
        }
    }
}

pub struct Intoer<I, O: From<I> + Clone> {
    pub input: NodeConfigInputPort<I>,
    pub output: NodeConfigOutputPort<O>,
}

impl<I, O: From<I> + Clone> Intoer<I, O> {
    pub fn new() -> Self {
        Self {
            input: NodeConfigInputPort::new(),
            output: NodeConfigOutputPort::new(),
        }
    }
}

impl<I, O: From<I> + Clone> Default for Intoer<I, O> {
    fn default() -> Self {
        Self::new()
    }
}

// TODO: Can + Send + 'static be removed?
impl<I: Send + 'static, O: From<I> + Clone + Send + 'static> NodeConfig for Intoer<I, O> {
    fn into_runner(self: Box<Self>) -> Box<dyn NodeRunner + Send> {
        Box::new(IntoerRunner {
            input: self.input.into(),
            output: self.output.into(),
        })
    }
}
