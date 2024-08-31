use conductor_core::{
    ports::{NodeConfigInputPort, NodeConfigOutputPort, NodeRunnerInputPort, NodeRunnerOutputPort},
    NodeConfig, NodeRunner,
};
use std::{ops::Add, sync::mpsc::TryRecvError};

struct AdderRunner<O, I1, I2>
where
    O: Clone,
    I1: Add<I2, Output = O> + Clone,
    I2: Clone,
{
    input1_cache: Option<I1>,
    input2_cache: Option<I2>,

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
    fn run(mut self: Box<Self>) {
        self.input1_cache = Some(self.input1.recv().unwrap());
        self.input2_cache = Some(self.input2.recv().unwrap());

        self.output.send(
            &(self.input1_cache.clone().expect("must exist")
                + self.input2_cache.clone().expect("must exist")),
        );

        loop {
            let input1 = self.input1.try_recv();
            let input2 = self.input2.try_recv();

            let (input1, input2) = match (input1, input2) {
                (Ok(input1), Ok(input2)) => {
                    self.input1_cache = Some(input1.clone());
                    self.input2_cache = Some(input2.clone());

                    (input1, input2)
                }
                (Ok(input1), Err(TryRecvError::Empty)) => {
                    self.input1_cache = Some(input1.clone());

                    (input1, self.input2_cache.clone().expect("must exist"))
                }
                (Err(TryRecvError::Empty), Ok(input2)) => {
                    self.input2_cache = Some(input2.clone());

                    (self.input1_cache.clone().expect("must exist"), input2)
                }
                (Err(TryRecvError::Empty), Err(TryRecvError::Empty)) => continue,
                _ => panic!(),
            };

            self.output.send(&(input1 + input2));
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
            input1_cache: None,
            input2_cache: None,

            input1: self.input1.into(),
            input2: self.input2.into(),
            output: self.output.into(),
        })
    }
}
