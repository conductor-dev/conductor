use conductor_core::{
    ports::{NodeConfigInputPort, NodeConfigOutputPort, NodeRunnerInputPort, NodeRunnerOutputPort},
    receive, NodeConfig, NodeRunner,
};

struct DownsampleRunner<T: Clone> {
    input: NodeRunnerInputPort<T>,
    factor: NodeRunnerInputPort<usize>,

    output: NodeRunnerOutputPort<T>,
}

impl<T: Clone> NodeRunner for DownsampleRunner<T> {
    fn run(self: Box<Self>) {
        let mut counter: usize = 0;

        let mut factor = self.factor.recv();

        loop {
            receive! {
                (self.input): value => {
                    counter %= factor;

                    if counter == 0 {
                        self.output.send(&value);
                    }

                    counter += 1;
                },
                (self.factor): new_factor => {
                    factor = new_factor;
                },
            };
        }
    }
}

pub struct Downsample<T: Clone> {
    pub input: NodeConfigInputPort<T>,
    pub factor: NodeConfigInputPort<usize>,

    pub output: NodeConfigOutputPort<T>,
}

impl<T: Clone> Downsample<T> {
    pub fn new() -> Self {
        Self {
            input: NodeConfigInputPort::new(),
            factor: NodeConfigInputPort::new(),
            output: NodeConfigOutputPort::new(),
        }
    }
}

impl<T: Clone> Default for Downsample<T> {
    fn default() -> Self {
        Self::new()
    }
}

// TODO: Can + Send + 'static be removed?
impl<T: Clone + Send + 'static> NodeConfig for Downsample<T> {
    fn into_runner(self: Box<Self>) -> Box<dyn NodeRunner + Send> {
        Box::new(DownsampleRunner {
            input: self.input.into(),
            factor: self.factor.into(),
            output: self.output.into(),
        })
    }
}
