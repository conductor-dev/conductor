use conductor_core::{
    ports::{NodeConfigInputPort, NodeConfigOutputPort, NodeRunnerInputPort, NodeRunnerOutputPort},
    receive, NodeConfig, NodeRunner,
};
use std::ops::Add;

macro_rules! implement_binary_arithmetic_operation {
    ($name:ident, $runner_name:ident, $trait:ident, $function:ident) => {
        struct $runner_name<I1, I2, O>
        where
            I1: $trait<I2, Output = O> + Clone,
            I2: Clone,
            O: Clone,
        {
            input1: NodeRunnerInputPort<I1>,
            input2: NodeRunnerInputPort<I2>,
            output: NodeRunnerOutputPort<O>,
        }

        impl<I1, I2, O> NodeRunner for $runner_name<I1, I2, O>
        where
            I1: $trait<I2, Output = O> + Clone,
            I2: Clone,
            O: Clone,
        {
            fn run(self: Box<Self>) {
                let mut input1_cache = self.input1.recv().unwrap();
                let mut input2_cache = self.input2.recv().unwrap();

                self.output
                    .send(&(input1_cache.clone().$function(input2_cache.clone())));

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
                        .send(&(input1_cache.clone().$function(input2_cache.clone())));
                }
            }
        }

        pub struct $name<I1, I2, O>
        where
            I1: $trait<I2, Output = O> + Clone,
            I2: Clone,
            O: Clone,
        {
            pub input1: NodeConfigInputPort<I1>,
            pub input2: NodeConfigInputPort<I2>,
            pub output: NodeConfigOutputPort<O>,
        }

        impl<I1, I2, O> $name<I1, I2, O>
        where
            I1: $trait<I2, Output = O> + Clone,
            I2: Clone,
            O: Clone,
        {
            pub fn new() -> Self {
                Self {
                    input1: NodeConfigInputPort::new(),
                    input2: NodeConfigInputPort::new(),
                    output: NodeConfigOutputPort::new(),
                }
            }
        }

        impl<I1, I2, O> Default for $name<I1, I2, O>
        where
            I1: $trait<I2, Output = O> + Clone,
            I2: Clone,
            O: Clone,
        {
            fn default() -> Self {
                Self::new()
            }
        }

        impl<I1, I2, O> NodeConfig for $name<I1, I2, O>
        where
            I1: $trait<I2, Output = O> + Send + Clone + 'static,
            I2: Send + Clone + 'static,
            O: Clone + Send + 'static,
        {
            fn into_runner(self: Box<Self>) -> Box<dyn NodeRunner + Send> {
                Box::new($runner_name {
                    input1: self.input1.into(),
                    input2: self.input2.into(),
                    output: self.output.into(),
                })
            }
        }
    };
}

implement_binary_arithmetic_operation!(Adder, AdderRunner, Add, add);
