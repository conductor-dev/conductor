use conductor_core::{
    buffer::CircularBuffer,
    ports::{NodeConfigInputPort, NodeConfigOutputPort, NodeRunnerInputPort, NodeRunnerOutputPort},
    NodeConfig, NodeRunner,
};
use rustfft::{num_complex::Complex, Fft, FftPlanner};
use std::{f64::consts::PI, sync::Arc};

struct FFTRunner<T: Into<f64>> {
    buffer: CircularBuffer<f64>,
    fft: Arc<dyn Fft<f64>>,
    scratch: Vec<Complex<f64>>,

    input: NodeRunnerInputPort<T>,
    output: NodeRunnerOutputPort<String>,
}

impl<T: Into<f64>> FFTRunner<T> {
    fn compute_fft(&mut self) -> Vec<f64> {
        let mut input: Vec<Complex<f64>> = self
            .buffer
            .iter()
            .enumerate()
            .map(|(i, &x)| {
                // Hanning window
                Complex::new(
                    x * 0.5
                        * (1.0 - f64::cos(2.0 * PI * i as f64 / (self.buffer.size() - 1) as f64)),
                    0.0,
                )
            })
            .collect();

        self.fft.process_with_scratch(&mut input, &mut self.scratch);

        input
            .iter()
            // Second half is a mirror of the first half
            .take(self.buffer.size() / 2)
            .map(|c| c.norm())
            .collect()
    }
}

impl<T: Into<f64>> NodeRunner for FFTRunner<T> {
    fn run(mut self: Box<Self>) {
        loop {
            self.buffer.push(self.input.recv().unwrap().into());

            if !self.buffer.is_filled() {
                continue;
            }

            let fft = self.compute_fft();

            let mut bins: Vec<(usize, f64)> = fft.into_iter().enumerate().collect();

            bins.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            let top_bins = &bins[0..3];

            self.output.send(&format!("{:?}", top_bins));
        }
    }
}

pub struct FFT<T: Into<f64>> {
    window_size: usize,
    pub input: NodeConfigInputPort<T>,
    pub output: NodeConfigOutputPort<String>,
}

impl<T: Into<f64>> FFT<T> {
    pub fn new(window_size: usize) -> Self {
        Self {
            window_size,
            input: NodeConfigInputPort::new(),
            output: NodeConfigOutputPort::new(),
        }
    }
}

// TODO: Can + Send + 'static be removed?
impl<T: Into<f64> + Send + 'static> NodeConfig for FFT<T> {
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
