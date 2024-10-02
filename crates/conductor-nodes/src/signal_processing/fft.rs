use conductor_core::{
    buffer::CircularBuffer,
    ports::{NodeConfigInputPort, NodeConfigOutputPort, NodeRunnerInputPort, NodeRunnerOutputPort},
    NodeConfig, NodeRunner,
};
use rustfft::{num_complex::Complex, Fft, FftPlanner};
use std::sync::Arc;

pub type FFTPacket = Vec<Complex<f32>>;

struct FFTRunner<T: Into<f32>> {
    buffer: CircularBuffer<f32>,
    fft: Arc<dyn Fft<f32>>,
    scratch: Vec<Complex<f32>>,

    input: NodeRunnerInputPort<T>,
    output: NodeRunnerOutputPort<FFTPacket>,
}

impl<T: Into<f32>> FFTRunner<T> {
    #[inline]
    fn compute_fft(&mut self) -> Vec<Complex<f32>> {
        let mut input = self
            .buffer
            .iter()
            .map(|x| Complex::new(*x, 0.0))
            .collect::<Vec<_>>();

        self.fft.process_with_scratch(&mut input, &mut self.scratch);

        input
            .into_iter()
            .map(|x| x / self.buffer.len() as f32)
            .collect()
    }
}

impl<T: Into<f32>> NodeRunner for FFTRunner<T> {
    fn run(mut self: Box<Self>) {
        loop {
            self.buffer.push(self.input.recv().unwrap().into());

            if !self.buffer.is_filled() {
                continue;
            }

            let fft = self.compute_fft();

            self.output.send(&fft);
        }
    }
}

pub struct FFT<T: Into<f32>> {
    window_size: usize,
    pub input: NodeConfigInputPort<T>,
    pub output: NodeConfigOutputPort<FFTPacket>,
}

impl<T: Into<f32>> FFT<T> {
    pub fn new(window_size: usize) -> Self {
        Self {
            window_size,
            input: NodeConfigInputPort::new(),
            output: NodeConfigOutputPort::new(),
        }
    }
}

// TODO: Can + Send + 'static be removed?
impl<T: Into<f32> + Send + 'static> NodeConfig for FFT<T> {
    fn into_runner(self: Box<Self>) -> Box<dyn NodeRunner + Send> {
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(self.window_size);

        Box::new(FFTRunner {
            buffer: CircularBuffer::new(self.window_size),
            fft,
            scratch: vec![Complex::default(); self.window_size],
            input: self.input.into(),
            output: self.output.into(),
        })
    }
}

struct InverseFFTRunner<T: From<f32> + Clone> {
    fft: Arc<dyn Fft<f32>>,
    scratch: Vec<Complex<f32>>,

    input: NodeRunnerInputPort<FFTPacket>,
    output: NodeRunnerOutputPort<T>,
}

impl<T: From<f32> + Clone> NodeRunner for InverseFFTRunner<T> {
    fn run(mut self: Box<Self>) {
        loop {
            let mut fft = self.input.recv().unwrap();

            self.fft.process_with_scratch(&mut fft, &mut self.scratch);

            // TODO: Check if pop() is the right method to use here
            let output = fft.pop().unwrap().re;

            self.output.send(&output.into());
        }
    }
}

pub struct InverseFFT<T: From<f32> + Clone> {
    window_size: usize,
    pub input: NodeConfigInputPort<FFTPacket>,
    pub output: NodeConfigOutputPort<T>,
}

impl<T: From<f32> + Clone> InverseFFT<T> {
    pub fn new(window_size: usize) -> Self {
        Self {
            window_size,
            input: NodeConfigInputPort::new(),
            output: NodeConfigOutputPort::new(),
        }
    }
}

// TODO: Can + Send + 'static be removed?
impl<T: From<f32> + Clone + Send + 'static> NodeConfig for InverseFFT<T> {
    fn into_runner(self: Box<Self>) -> Box<dyn NodeRunner + Send> {
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_inverse(self.window_size);

        Box::new(InverseFFTRunner {
            fft,
            scratch: vec![Complex::default(); self.window_size],
            input: self.input.into(),
            output: self.output.into(),
        })
    }
}
