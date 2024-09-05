use conductor_core::{
    ports::{NodeConfigInputPort, NodeConfigOutputPort, NodeRunnerInputPort, NodeRunnerOutputPort},
    receive, NodeConfig, NodeRunner,
};
use std::ops::Add;

struct AdderRunner<O, I1, I2>
where
    O: Clone,
    I1: Add<I2, Output = O> + Clone,
    I2: Clone,
{
    input1: NodeRunnerInputPort<I1>,
    input2: NodeRunnerInputPort<I2>,
    output: NodeRunnerOutputPort<O>,
}

impl<O, I1, I2> NodeRunner for AdderRunner<O, I1, I2>
where
    O: Clone,
    I1: Add<I2, Output = O> + Clone,
    I2: Clone,
{
    fn run(self: Box<Self>) {
        let mut input1_cache = self.input1.recv().unwrap();
        let mut input2_cache = self.input2.recv().unwrap();

        self.output
            .send(&(input1_cache.clone() + input2_cache.clone()));

        loop {
            receive! {
                (self.input1): msg => {
                    input1_cache = msg;
                },
                (self.input2): msg => {
                    input2_cache = msg;
                },
            };

            self.output
                .send(&(input1_cache.clone() + input2_cache.clone()));
        }
    }
}

pub struct Adder<O, I1, I2>
where
    O: Clone,
    I1: Add<I2, Output = O> + Clone,
    I2: Clone,
{
    pub input1: NodeConfigInputPort<I1>,
    pub input2: NodeConfigInputPort<I2>,
    pub output: NodeConfigOutputPort<O>,
}

impl<O, I1, I2> Adder<O, I1, I2>
where
    O: Clone,
    I1: Add<I2, Output = O> + Clone,
    I2: Clone,
{
    pub fn new() -> Self {
        Self {
            input1: NodeConfigInputPort::new(),
            input2: NodeConfigInputPort::new(),
            output: NodeConfigOutputPort::new(),
        }
    }
}

impl<O, I1, I2> Default for Adder<O, I1, I2>
where
    O: Clone,
    I1: Add<I2, Output = O> + Clone,
    I2: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<O, I1, I2> NodeConfig for Adder<O, I1, I2>
where
    O: Clone + Send + 'static,
    I1: Add<I2, Output = O> + Send + Clone + 'static,
    I2: Send + Clone + 'static,
{
    fn into_runner(self: Box<Self>) -> Box<dyn NodeRunner + Send> {
        Box::new(AdderRunner {
            input1: self.input1.into(),
            input2: self.input2.into(),
            output: self.output.into(),
        })
    }
}
