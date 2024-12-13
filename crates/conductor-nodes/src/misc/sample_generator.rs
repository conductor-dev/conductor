use conductor_core::{
    ports::{NodeConfigInputPort, NodeConfigOutputPort, NodeRunnerInputPort, NodeRunnerOutputPort},
    timer::set_interval,
    NodeConfig, NodeRunner,
};
use std::{ops::Add, time::Duration};

struct SampleGeneratorRunner<O: Clone + Add<O, Output = O>> {
    initial_value: O,

    output: NodeRunnerOutputPort<O>,
    sample_rate: NodeRunnerInputPort<usize>,
    step: NodeRunnerInputPort<O>,
}

impl<O: Clone + Add<O, Output = O>> NodeRunner for SampleGeneratorRunner<O> {
    fn run(self: Box<Self>) {
        let sample_rate = self.sample_rate.recv();
        let step = self.step.recv();

        let mut current_value = self.initial_value;
        let seconds_per_sample = Duration::from_secs_f64(1.0 / (sample_rate as f64));

        set_interval(seconds_per_sample, || {
            self.output.send(&current_value);

            current_value = current_value.clone() + step.clone();
        })
    }
}

pub struct SampleGenerator<O: Clone + Add<O, Output = O>> {
    initial_value: O,

    pub output: NodeConfigOutputPort<O>,
    pub sample_rate: NodeConfigInputPort<usize>,
    pub step: NodeConfigInputPort<O>,
}

impl<O: Clone + Add<O, Output = O>> SampleGenerator<O> {
    pub fn new(initial_value: O) -> Self {
        Self {
            initial_value,
            output: NodeConfigOutputPort::new(),
            sample_rate: NodeConfigInputPort::new(),
            step: NodeConfigInputPort::new(),
        }
    }
}

impl<O: Clone + Add<O, Output = O> + Send + 'static> NodeConfig for SampleGenerator<O> {
    fn into_runner(self: Box<Self>) -> Box<dyn NodeRunner + Send> {
        Box::new(SampleGeneratorRunner {
            initial_value: self.initial_value,

            output: self.output.into(),
            sample_rate: self.sample_rate.into(),
            step: self.step.into(),
        })
    }
}
