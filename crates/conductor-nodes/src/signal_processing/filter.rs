use super::fft::FFTPacket;
use conductor_core::{
    ports::{NodeConfigInputPort, NodeConfigOutputPort, NodeRunnerInputPort, NodeRunnerOutputPort},
    NodeConfig, NodeRunner,
};
use rustfft::num_complex::Complex;

struct BandpassFilterRunner {
    input: NodeRunnerInputPort<FFTPacket>,
    sample_rate: NodeRunnerInputPort<usize>,
    output: NodeRunnerOutputPort<FFTPacket>,
    low_cutoff: f32,
    high_cutoff: f32,
}

impl NodeRunner for BandpassFilterRunner {
    fn run(self: Box<Self>) {
        let sample_rate = self.sample_rate.recv().unwrap();

        loop {
            let mut input = self.input.recv().unwrap();

            let full_size = input.len();
            let half_size = input.len() / 2;

            let low_cutoff_index =
                ((self.low_cutoff * full_size as f32) / sample_rate as f32) as usize;

            for value in input.iter_mut().take(low_cutoff_index) {
                *value = Complex::new(0.0, 0.0);
            }

            for value in input
                .iter_mut()
                .take(full_size)
                .skip(full_size - low_cutoff_index)
            {
                *value = Complex::new(0.0, 0.0);
            }

            let high_cutoff_index =
                ((self.high_cutoff * full_size as f32) / sample_rate as f32) as usize;

            for value in input.iter_mut().take(half_size).skip(high_cutoff_index) {
                *value = Complex::new(0.0, 0.0);
            }

            for value in input.iter_mut().skip(half_size).take(high_cutoff_index) {
                *value = Complex::new(0.0, 0.0);
            }

            self.output.send(&input);
        }
    }
}

pub struct BandpassFilter {
    pub input: NodeConfigInputPort<FFTPacket>,
    pub sample_rate: NodeConfigInputPort<usize>,
    pub output: NodeConfigOutputPort<FFTPacket>,
    low_cutoff: f32,
    high_cutoff: f32,
}

impl BandpassFilter {
    pub fn new(low_cutoff: f32, high_cutoff: f32) -> Self {
        Self {
            input: NodeConfigInputPort::new(),
            sample_rate: NodeConfigInputPort::new(),
            output: NodeConfigOutputPort::new(),
            low_cutoff,
            high_cutoff,
        }
    }
}

// TODO: Can + Send + 'static be removed?
impl NodeConfig for BandpassFilter {
    fn into_runner(self: Box<Self>) -> Box<dyn NodeRunner + Send> {
        Box::new(BandpassFilterRunner {
            input: self.input.into(),
            sample_rate: self.sample_rate.into(),
            output: self.output.into(),
            low_cutoff: self.low_cutoff,
            high_cutoff: self.high_cutoff,
        })
    }
}

struct LowpassFilterRunner {
    input: NodeRunnerInputPort<FFTPacket>,
    sample_rate: NodeRunnerInputPort<usize>,
    output: NodeRunnerOutputPort<FFTPacket>,
    frequency: f32,
}

impl NodeRunner for LowpassFilterRunner {
    fn run(self: Box<Self>) {
        let sample_rate = self.sample_rate.recv().unwrap();

        loop {
            let mut input = self.input.recv().unwrap();

            let full_size = input.len();
            let half_size = input.len() / 2;

            let cutoff_index = ((self.frequency * full_size as f32) / sample_rate as f32) as usize;

            for value in input.iter_mut().skip(cutoff_index).take(half_size) {
                *value = Complex::new(0.0, 0.0);
            }

            self.output.send(&input);
        }
    }
}

pub struct LowpassFilter {
    pub input: NodeConfigInputPort<FFTPacket>,
    pub sample_rate: NodeConfigInputPort<usize>,
    pub output: NodeConfigOutputPort<FFTPacket>,
    frequency: f32,
}

impl LowpassFilter {
    pub fn new(frequency: f32) -> Self {
        Self {
            input: NodeConfigInputPort::new(),
            sample_rate: NodeConfigInputPort::new(),
            output: NodeConfigOutputPort::new(),
            frequency,
        }
    }
}

// TODO: Can + Send + 'static be removed?
impl NodeConfig for LowpassFilter {
    fn into_runner(self: Box<Self>) -> Box<dyn NodeRunner + Send> {
        Box::new(LowpassFilterRunner {
            input: self.input.into(),
            sample_rate: self.sample_rate.into(),
            output: self.output.into(),
            frequency: self.frequency,
        })
    }
}

struct HighpassFilterRunner {
    input: NodeRunnerInputPort<FFTPacket>,
    sample_rate: NodeRunnerInputPort<usize>,
    output: NodeRunnerOutputPort<FFTPacket>,
    frequency: f32,
}

impl NodeRunner for HighpassFilterRunner {
    fn run(self: Box<Self>) {
        let sample_rate = self.sample_rate.recv().unwrap();

        loop {
            let mut input = self.input.recv().unwrap();

            let full_size = input.len();

            let cutoff_index = ((self.frequency * full_size as f32) / sample_rate as f32) as usize;

            for value in input.iter_mut().take(cutoff_index) {
                *value = Complex::new(0.0, 0.0);
            }

            for value in input
                .iter_mut()
                .take(full_size)
                .skip(full_size - cutoff_index)
            {
                *value = Complex::new(0.0, 0.0);
            }

            self.output.send(&input);
        }
    }
}

pub struct HighpassFilter {
    pub input: NodeConfigInputPort<FFTPacket>,
    pub sample_rate: NodeConfigInputPort<usize>,
    pub output: NodeConfigOutputPort<FFTPacket>,
    frequency: f32,
}

impl HighpassFilter {
    pub fn new(frequency: f32) -> Self {
        Self {
            input: NodeConfigInputPort::new(),
            sample_rate: NodeConfigInputPort::new(),
            output: NodeConfigOutputPort::new(),
            frequency,
        }
    }
}

// TODO: Can + Send + 'static be removed?
impl NodeConfig for HighpassFilter {
    fn into_runner(self: Box<Self>) -> Box<dyn NodeRunner + Send> {
        Box::new(HighpassFilterRunner {
            input: self.input.into(),
            sample_rate: self.sample_rate.into(),
            output: self.output.into(),
            frequency: self.frequency,
        })
    }
}
