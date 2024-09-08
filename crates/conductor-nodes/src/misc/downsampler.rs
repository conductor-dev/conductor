use conductor_core::{
    ports::{NodeConfigInputPort, NodeConfigOutputPort, NodeRunnerInputPort, NodeRunnerOutputPort},
    NodeConfig, NodeRunner,
};

struct DownsamplerRunner<T: Clone> {
    input: NodeRunnerInputPort<T>,
    output: NodeRunnerOutputPort<T>,
    factor: usize,
}

impl<T: Clone> NodeRunner for DownsamplerRunner<T> {
    fn run(self: Box<Self>) {
        let mut counter: usize = 0;

        loop {
            let value = self.input.recv().unwrap();

            counter %= self.factor;
            if counter == 0 {
                self.output.send(&value);
            }
            counter += 1;
        }
    }
}

pub struct Downsampler<T: Clone> {
    pub input: NodeConfigInputPort<T>,
    pub output: NodeConfigOutputPort<T>,
    pub factor: usize,
}

impl<T: Clone> Downsampler<T> {
    pub fn new(factor: usize) -> Self {
        Self {
            input: NodeConfigInputPort::<T>::new(),
            output: NodeConfigOutputPort::<T>::new(),
            factor,
        }
    }
}

// TODO: Can + Send + 'static be removed?
impl<T: Clone + Send + 'static> NodeConfig for Downsampler<T> {
    fn into_runner(self: Box<Self>) -> Box<dyn NodeRunner + Send> {
        Box::new(DownsamplerRunner {
            input: self.input.into(),
            output: self.output.into(),
            factor: self.factor,
        })
    }
}
