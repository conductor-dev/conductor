use conductor_core::{
    ports::{NodeConfigInputPort, NodeConfigOutputPort, NodeRunnerInputPort, NodeRunnerOutputPort},
    NodeConfig, NodeRunner,
};

struct LambdaRunner<I, O: Clone> {
    input: NodeRunnerInputPort<I>,
    output: NodeRunnerOutputPort<O>,
    function: Box<dyn Fn(I) -> O + Send>,
}

impl<T, O: Clone> NodeRunner for LambdaRunner<T, O> {
    fn run(self: Box<Self>) {
        loop {
            let input = self.input.recv();

            let output = (self.function)(input);

            self.output.send(&output);
        }
    }
}

pub struct Lambda<I, O: Clone> {
    pub input: NodeConfigInputPort<I>,
    pub output: NodeConfigOutputPort<O>,
    function: Box<dyn Fn(I) -> O + Send>,
}

impl<I, O: Clone> Lambda<I, O> {
    pub fn new<F: Fn(I) -> O + Send + 'static>(function: F) -> Self {
        Self {
            input: NodeConfigInputPort::new(),
            output: NodeConfigOutputPort::new(),
            function: Box::new(function),
        }
    }
}

// TODO: Can + Send + 'static be removed?
impl<I: Send + 'static, O: Clone + Send + 'static> NodeConfig for Lambda<I, O> {
    fn into_runner(self: Box<Self>) -> Box<dyn NodeRunner + Send> {
        Box::new(LambdaRunner {
            input: self.input.into(),
            output: self.output.into(),
            function: self.function,
        })
    }
}
