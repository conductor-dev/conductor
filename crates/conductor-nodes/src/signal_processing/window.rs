use conductor_core::{
    ports::{NodeConfigInputPort, NodeConfigOutputPort, NodeRunnerInputPort, NodeRunnerOutputPort},
    NodeConfig, NodeRunner,
};

pub enum WindowType {
    Hann,
    Hamming,
}

struct WindowRunner<T: Into<f32>> {
    input: NodeRunnerInputPort<Vec<T>>,
    output: NodeRunnerOutputPort<Vec<f32>>,
    window_type: WindowType,
}

impl<T: Into<f32>> NodeRunner for WindowRunner<T> {
    fn run(self: Box<Self>) {
        fn cosine_sum_window<T: Into<f32>>(values: Vec<T>, alpha_zero: f32) -> Vec<f32> {
            let length = values.len();

            values
                .into_iter()
                .enumerate()
                .map(|(i, value)| {
                    value.into()
                        * alpha_zero
                        * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / length as f32).cos())
                })
                .collect()
        }

        let alpha_zero = match self.window_type {
            WindowType::Hann => 0.5,
            WindowType::Hamming => 25.0 / 46.0,
        };

        loop {
            let values = self.input.recv().unwrap();

            let windowed_values = cosine_sum_window(values, alpha_zero);

            self.output.send(&windowed_values);
        }
    }
}

pub struct Window<T: Into<f32>> {
    pub input: NodeConfigInputPort<Vec<T>>,
    pub output: NodeConfigOutputPort<Vec<f32>>,

    window_type: WindowType,
}

impl<T: Into<f32>> Window<T> {
    pub fn new(window_type: WindowType) -> Self {
        Self {
            input: NodeConfigInputPort::new(),
            output: NodeConfigOutputPort::new(),
            window_type,
        }
    }
}

// TODO: Can + Send + 'static be removed?
impl<T: Into<f32> + Send + 'static> NodeConfig for Window<T> {
    fn into_runner(self: Box<Self>) -> Box<dyn NodeRunner + Send> {
        Box::new(WindowRunner {
            input: self.input.into(),
            output: self.output.into(),
            window_type: self.window_type,
        })
    }
}
