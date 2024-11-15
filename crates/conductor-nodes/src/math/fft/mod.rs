pub mod window;

use conductor_core::{
    ports::{NodeConfigInputPort, NodeConfigOutputPort, NodeRunnerInputPort, NodeRunnerOutputPort},
    NodeConfig, NodeRunner,
};
use rustfft::{num_complex::Complex, Fft, FftPlanner};
use std::sync::Arc;

pub type FFTPacket = Vec<Complex<f32>>;

struct FFTRunner<T: Into<f32>> {
    input: NodeRunnerInputPort<Vec<T>>,
    output: NodeRunnerOutputPort<FFTPacket>,
}

impl<T: Into<f32>> NodeRunner for FFTRunner<T> {
    fn run(self: Box<Self>) {
        fn compute_fft<T: Into<f32>>(
            buffer: Vec<T>,
            fft: &Arc<dyn Fft<f32>>,
            scratch: &mut [Complex<f32>],
        ) -> Vec<Complex<f32>> {
            let buffer_length = buffer.len();

            let mut input = buffer
                .into_iter()
                .map(|x| Complex::<f32>::new(x.into(), 0.0))
                .collect::<Vec<_>>();

            fft.process_with_scratch(&mut input, scratch);

            input
                .into_iter()
                .map(|x| x / buffer_length as f32)
                .collect()
        }

        let mut planner = FftPlanner::new();

        let mut buffer = self.input.recv().unwrap();
        let mut scratch = vec![Complex::default(); buffer.len()];

        let mut fft = planner.plan_fft_forward(buffer.len());

        loop {
            let fft_output = compute_fft(buffer, &fft, &mut scratch);

            self.output.send(&fft_output);

            buffer = self.input.recv().unwrap();

            if fft.len() != buffer.len() {
                fft = planner.plan_fft_forward(buffer.len());
                scratch.resize(buffer.len(), Complex::default());
            }
        }
    }
}

pub struct FFT<T: Into<f32>> {
    pub input: NodeConfigInputPort<Vec<T>>,
    pub output: NodeConfigOutputPort<FFTPacket>,
}

impl<T: Into<f32>> FFT<T> {
    pub fn new() -> Self {
        Self {
            input: NodeConfigInputPort::new(),
            output: NodeConfigOutputPort::new(),
        }
    }
}

impl<T: Into<f32>> Default for FFT<T> {
    fn default() -> Self {
        Self::new()
    }
}

// TODO: Can + Send + 'static be removed?
impl<T: Into<f32> + Send + 'static> NodeConfig for FFT<T> {
    fn into_runner(self: Box<Self>) -> Box<dyn NodeRunner + Send> {
        Box::new(FFTRunner {
            input: self.input.into(),
            output: self.output.into(),
        })
    }
}

struct InverseFFTRunner {
    input: NodeRunnerInputPort<FFTPacket>,
    output: NodeRunnerOutputPort<Vec<f32>>,
}

impl NodeRunner for InverseFFTRunner {
    fn run(self: Box<Self>) {
        let mut planner = FftPlanner::new();

        let mut fft_input = self.input.recv().unwrap();
        let mut scratch = vec![Complex::default(); fft_input.len()];

        let mut fft = planner.plan_fft_inverse(fft_input.len());

        loop {
            fft.process_with_scratch(&mut fft_input, &mut scratch);

            let fft_output = fft_input.into_iter().map(|value| value.re).collect();

            self.output.send(&fft_output);

            fft_input = self.input.recv().unwrap();

            if fft.len() != fft_input.len() {
                fft = planner.plan_fft_inverse(fft_input.len());
                scratch.resize(fft_input.len(), Complex::default());
            }
        }
    }
}

pub struct InverseFFT {
    pub input: NodeConfigInputPort<FFTPacket>,
    pub output: NodeConfigOutputPort<Vec<f32>>,
}

impl InverseFFT {
    pub fn new() -> Self {
        Self {
            input: NodeConfigInputPort::new(),
            output: NodeConfigOutputPort::new(),
        }
    }
}

impl Default for InverseFFT {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeConfig for InverseFFT {
    fn into_runner(self: Box<Self>) -> Box<dyn NodeRunner + Send> {
        Box::new(InverseFFTRunner {
            input: self.input.into(),
            output: self.output.into(),
        })
    }
}
