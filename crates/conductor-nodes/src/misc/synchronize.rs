use conductor_core::{
    ports::{NodeConfigInputPort, NodeConfigOutputPort, NodeRunnerInputPort, NodeRunnerOutputPort},
    NodeConfig, NodeRunner,
};

struct SynchronizeRunner<T1: Clone, T2: Clone> {
    input1: NodeRunnerInputPort<T1>,
    input2: NodeRunnerInputPort<T2>,

    output1: NodeRunnerOutputPort<T1>,
    output2: NodeRunnerOutputPort<T2>,
}

impl<T1: Clone, T2: Clone> NodeRunner for SynchronizeRunner<T1, T2> {
    fn run(self: Box<Self>) {
        loop {
            let value1 = self.input1.recv().unwrap();
            let value2 = self.input2.recv().unwrap();

            self.output1.send(&value1);
            self.output2.send(&value2);
        }
    }
}

pub struct Synchronize<T1: Clone, T2: Clone> {
    pub input1: NodeConfigInputPort<T1>,
    pub input2: NodeConfigInputPort<T2>,

    pub output1: NodeConfigOutputPort<T1>,
    pub output2: NodeConfigOutputPort<T2>,
}

impl<T1: Clone, T2: Clone> Synchronize<T1, T2> {
    pub fn new() -> Self {
        Self {
            input1: NodeConfigInputPort::new(),
            input2: NodeConfigInputPort::new(),
            output1: NodeConfigOutputPort::new(),
            output2: NodeConfigOutputPort::new(),
        }
    }
}

impl<T1: Clone, T2: Clone> Default for Synchronize<T1, T2> {
    fn default() -> Self {
        Self::new()
    }
}

// TODO: Can + Send + 'static be removed?
impl<T1: Clone + Send + 'static, T2: Clone + Send + 'static> NodeConfig for Synchronize<T1, T2> {
    fn into_runner(self: Box<Self>) -> Box<dyn NodeRunner + Send> {
        Box::new(SynchronizeRunner {
            input1: self.input1.into(),
            input2: self.input2.into(),
            output1: self.output1.into(),
            output2: self.output2.into(),
        })
    }
}
