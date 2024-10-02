pub mod arihtmetic;
pub mod immediate;
pub mod norm;
pub mod trigonometry;

#[macro_export]
macro_rules! implement_unary_trait_operation {
    ($name:ident, $runner_name:ident, $trait:ident, $function:ident) => {
        struct $runner_name<I, O>
        where
            I: $trait<Output = O>,
            O: Clone,
        {
            input: NodeRunnerInputPort<I>,
            output: NodeRunnerOutputPort<O>,
        }

        impl<I, O> NodeRunner for $runner_name<I, O>
        where
            I: $trait<Output = O>,
            O: Clone,
        {
            fn run(self: Box<Self>) {
                loop {
                    let input = self.input.recv().unwrap();

                    self.output.send(&input.$function());
                }
            }
        }

        pub struct $name<I, O>
        where
            I: $trait<Output = O>,
            O: Clone,
        {
            pub input: NodeConfigInputPort<I>,
            pub output: NodeConfigOutputPort<O>,
        }

        impl<I, O> $name<I, O>
        where
            I: $trait<Output = O>,
            O: Clone,
        {
            pub fn new() -> Self {
                Self {
                    input: NodeConfigInputPort::new(),
                    output: NodeConfigOutputPort::new(),
                }
            }
        }

        impl<I, O> Default for $name<I, O>
        where
            I: $trait<Output = O>,
            O: Clone,
        {
            fn default() -> Self {
                Self::new()
            }
        }

        impl<I, O> NodeConfig for $name<I, O>
        where
            I: $trait<Output = O> + Send + Clone + 'static,
            O: Clone + Send + 'static,
        {
            fn into_runner(self: Box<Self>) -> Box<dyn NodeRunner + Send> {
                Box::new($runner_name {
                    input: self.input.into(),
                    output: self.output.into(),
                })
            }
        }
    };
}
