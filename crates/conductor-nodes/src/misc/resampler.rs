use conductor_core::{
    ports::{NodeConfigInputPort, NodeConfigOutputPort, NodeRunnerInputPort, NodeRunnerOutputPort},
    NodeConfig, NodeRunner,
};

struct ResamplerRunner {
    input: NodeRunnerInputPort<f32>,
    output: NodeRunnerOutputPort<f32>,
    input_sample_rate: NodeRunnerInputPort<usize>,
    output_sample_rate: NodeRunnerInputPort<usize>,
}

impl NodeRunner for ResamplerRunner {
    fn run(self: Box<Self>) {
        // TODO: support dynamically changing sample rates
        let input_sample_rate = self.input_sample_rate.recv().unwrap();
        let output_sample_rate = self.output_sample_rate.recv().unwrap();

        let mut buffer = Vec::new();
        let mut position = 0.0;
        let factor = input_sample_rate as f32 / output_sample_rate as f32;

        fn lerp(a: f32, b: f32, alpha: f32) -> f32 {
            a * (1.0 - alpha) + b * alpha
        }

        loop {
            let sample = self.input.recv().unwrap();
            buffer.push(sample);

            while position + factor.max(1.0) < buffer.len() as f32 {
                let index = position as usize;

                if index + 1 < buffer.len() {
                    self.output.send(&lerp(
                        buffer[index],
                        buffer[index + 1],
                        position - index as f32,
                    ));
                }

                position += factor;
            }

            let remove_up_to = position as usize;
            buffer.drain(0..remove_up_to);
            position -= remove_up_to as f32;
        }
    }
}

pub struct Resampler {
    pub input: NodeConfigInputPort<f32>,
    pub output: NodeConfigOutputPort<f32>,
    pub input_sample_rate: NodeConfigInputPort<usize>,
    pub output_sample_rate: NodeConfigInputPort<usize>,
}

impl Resampler {
    pub fn new() -> Self {
        Self {
            input: NodeConfigInputPort::new(),
            output: NodeConfigOutputPort::new(),
            input_sample_rate: NodeConfigInputPort::new(),
            output_sample_rate: NodeConfigInputPort::new(),
        }
    }
}

impl Default for Resampler {
    fn default() -> Self {
        Self::new()
    }
}

// TODO: Can + Send + 'static be removed?
impl NodeConfig for Resampler {
    fn into_runner(self: Box<Self>) -> Box<dyn NodeRunner + Send> {
        Box::new(ResamplerRunner {
            input: self.input.into(),
            output: self.output.into(),
            input_sample_rate: self.input_sample_rate.into(),
            output_sample_rate: self.output_sample_rate.into(),
        })
    }
}
