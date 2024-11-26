use conductor_core::{
    ports::{NodeConfigInputPort, NodeConfigOutputPort, NodeRunnerInputPort, NodeRunnerOutputPort},
    receive, NodeConfig, NodeRunner,
};

struct GateRunner<T: Clone> {
    condition: NodeRunnerInputPort<bool>,

    input: NodeRunnerInputPort<T>,

    output: NodeRunnerOutputPort<T>,
}

impl<T: Clone> NodeRunner for GateRunner<T> {
    fn run(self: Box<Self>) {
        let mut condition = false;

        loop {
            receive! {
                (self.condition): new_condition => condition = new_condition,
                (self.input): input => {
                    if condition {
                        self.output.send(&input);
                    }
                },
            }
        }
    }
}

pub struct Gate<T: Clone> {
    pub condition: NodeConfigInputPort<bool>,

    pub input: NodeConfigInputPort<T>,

    pub output: NodeConfigOutputPort<T>,
}

impl<T: Clone> Gate<T> {
    pub fn new() -> Self {
        Self {
            condition: NodeConfigInputPort::new(),
            input: NodeConfigInputPort::new(),
            output: NodeConfigOutputPort::new(),
        }
    }
}

impl<T: Clone> Default for Gate<T> {
    fn default() -> Self {
        Self::new()
    }
}

// TODO: Can + Send + 'static be removed?
impl<T: Clone + Send + 'static> NodeConfig for Gate<T> {
    fn into_runner(self: Box<Self>) -> Box<dyn NodeRunner + Send> {
        Box::new(GateRunner {
            condition: self.condition.into(),
            input: self.input.into(),
            output: self.output.into(),
        })
    }
}
