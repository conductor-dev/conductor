use conductor_core::{
    ports::{NodeConfigInputPort, NodeConfigOutputPort, NodeRunnerInputPort, NodeRunnerOutputPort},
    receive, NodeConfig, NodeRunner,
};

macro_rules! implement_comparison_operation {
    ($name:ident, $runner_name:ident, $trait:ident, $function:ident) => {
        struct $runner_name<I1, I2>
        where
            I1: $trait<I2>,
        {
            input1: NodeRunnerInputPort<I1>,
            input2: NodeRunnerInputPort<I2>,
            output: NodeRunnerOutputPort<bool>,
        }

        impl<I1, I2> NodeRunner for $runner_name<I1, I2>
        where
            I1: $trait<I2>,
        {
            fn run(self: Box<Self>) {
                let mut input1 = self.input1.recv();
                let mut input2 = self.input2.recv();

                loop {
                    let condition = input1.$function(&input2);

                    self.output.send(&condition);

                    receive! {
                        (self.input1): new_input1 => input1 = new_input1,
                        (self.input2): new_input2 => input2 = new_input2,
                    };
                }
            }
        }

        pub struct $name<I1, I2>
        where
            I1: $trait<I2>,
        {
            pub input1: NodeConfigInputPort<I1>,
            pub input2: NodeConfigInputPort<I2>,
            pub output: NodeConfigOutputPort<bool>,
        }

        impl<I1, I2> $name<I1, I2>
        where
            I1: $trait<I2>,
        {
            pub fn new() -> Self {
                Self {
                    input1: NodeConfigInputPort::new(),
                    input2: NodeConfigInputPort::new(),
                    output: NodeConfigOutputPort::new(),
                }
            }
        }

        impl<I1, I2> Default for $name<I1, I2>
        where
            I1: $trait<I2>,
        {
            fn default() -> Self {
                Self::new()
            }
        }

        // TODO: Can Send + 'static be removed?
        impl<I1: Send + 'static, I2: Send + 'static> NodeConfig for $name<I1, I2>
        where
            I1: $trait<I2>,
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

implement_comparison_operation!(GreaterThan, GreaterThanRunner, PartialOrd, gt);
implement_comparison_operation!(GreaterThanEqual, GreaterThanEqualRunner, PartialOrd, ge);
implement_comparison_operation!(LessThan, LessThanRunner, PartialOrd, lt);
implement_comparison_operation!(LessThanEqual, LessThanRunnerEqual, PartialOrd, le);
implement_comparison_operation!(Equal, EqualRunner, PartialEq, eq);
implement_comparison_operation!(NotEqual, NotEqualRunner, PartialEq, ne);
