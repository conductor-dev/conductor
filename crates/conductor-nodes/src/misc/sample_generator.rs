use conductor_core::{
    ports::{NodeConfigOutputPort, NodeRunnerOutputPort},
    NodeConfig, NodeRunner,
};
use std::time::{Duration, Instant};

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

struct SampleGeneratorRunner<O: Sample + Clone> {
    output: NodeRunnerOutputPort<O>,
    sample_rate: usize,
}

impl<O: Sample + Clone> NodeRunner for SampleGeneratorRunner<O> {
    fn run(self: Box<Self>) {
        let mut current_sample = 0;
        let seconds_per_sample = 1.0 / (self.sample_rate as f64);
        let mut last_time = Instant::now();

        loop {
            self.output
                .send(&O::sample(self.sample_rate, current_sample));

            current_sample += 1;

            while last_time.elapsed() < Duration::from_secs_f64(seconds_per_sample) {
                // TODO: Maybe use
                // std::hint::spin_loop();
                // or
                // thread::yield_now();
            }
            last_time = Instant::now();
        }
    }
}

pub struct SampleGenerator<O: Sample + Clone> {
    pub output: NodeConfigOutputPort<O>,
    sample_rate: usize,
}

impl<O: Sample + Clone> SampleGenerator<O> {
    pub fn new(sample_rate: usize) -> Self {
        Self {
            output: NodeConfigOutputPort::new(),
            sample_rate,
        }
    }
}

impl<O: Sample + Clone + Send + 'static> NodeConfig for SampleGenerator<O> {
    fn into_runner(self: Box<Self>) -> Box<dyn NodeRunner + Send> {
        Box::new(SampleGeneratorRunner {
            output: self.output.into(),
            sample_rate: self.sample_rate,
        })
    }
}
