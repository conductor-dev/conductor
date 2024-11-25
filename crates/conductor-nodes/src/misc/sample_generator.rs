use conductor_core::{
    ports::{NodeConfigInputPort, NodeConfigOutputPort, NodeRunnerInputPort, NodeRunnerOutputPort},
    timer::set_interval,
    NodeConfig, NodeRunner,
};
use std::time::Duration;

pub trait Sample {
    fn sample(sample_rate: usize, current_sample: usize) -> Self;
}

impl Sample for f64 {
    fn sample(sample_rate: usize, current_sample: usize) -> Self {
        (current_sample as f64) / (sample_rate as f64)
    }
}

impl Sample for f32 {
    fn sample(sample_rate: usize, current_sample: usize) -> Self {
        (current_sample as f32) / (sample_rate as f32)
    }
}

impl Sample for i64 {
    fn sample(sample_rate: usize, current_sample: usize) -> Self {
        (current_sample as i64) / (sample_rate as i64)
    }
}

impl Sample for i32 {
    fn sample(sample_rate: usize, current_sample: usize) -> Self {
        (current_sample as i32) / (sample_rate as i32)
    }
}

struct SampleGeneratorRunner<O: Sample + Clone> {
    output: NodeRunnerOutputPort<O>,
    sample_rate: NodeRunnerInputPort<usize>,
}

impl<O: Sample + Clone> NodeRunner for SampleGeneratorRunner<O> {
    fn run(self: Box<Self>) {
        let sample_rate = self.sample_rate.recv().unwrap();

        let mut current_sample = 0;
        let seconds_per_sample = Duration::from_secs_f64(1.0 / (sample_rate as f64));

        set_interval(seconds_per_sample, || {
            self.output.send(&O::sample(sample_rate, current_sample));

            current_sample += 1;
        })
    }
}

pub struct SampleGenerator<O: Sample + Clone> {
    pub output: NodeConfigOutputPort<O>,
    pub sample_rate: NodeConfigInputPort<usize>,
}

impl<O: Sample + Clone> SampleGenerator<O> {
    pub fn new() -> Self {
        Self {
            output: NodeConfigOutputPort::new(),
            sample_rate: NodeConfigInputPort::new(),
        }
    }
}

impl<O: Sample + Clone> Default for SampleGenerator<O> {
    fn default() -> Self {
        Self::new()
    }
}

impl<O: Sample + Clone + Send + 'static> NodeConfig for SampleGenerator<O> {
    fn into_runner(self: Box<Self>) -> Box<dyn NodeRunner + Send> {
        Box::new(SampleGeneratorRunner {
            output: self.output.into(),
            sample_rate: self.sample_rate.into(),
        })
    }
}
