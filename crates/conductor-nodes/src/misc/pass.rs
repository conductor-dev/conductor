use conductor_core::{
    node::{Node, SinkPort, SinkPortCell, SourcePort, SourcePortCell},
    runner::Runner,
};

pub struct PassRunner<T: Clone> {
    pub input: SinkPort<T>,
    pub output: SourcePort<T>,
}

impl<T: Clone> Runner for PassRunner<T> {
    fn run(&self) {
        loop {
            let value = self.input.recv().unwrap();
            self.output.send(&value);
        }
    }
}

pub struct Pass<T: Clone> {
    pub input: SinkPortCell<T>,
    pub output: SourcePortCell<T>,
}

impl<T: Clone> Pass<T> {
    pub fn new() -> Self {
        Self {
            input: SinkPortCell::<T>::new(),
            output: SourcePortCell::<T>::new(),
        }
    }
}

impl<T: Clone> Default for Pass<T> {
    fn default() -> Self {
        Self::new()
    }
}

// TODO: Can + Send + 'static be removed?
impl<T: Clone + Send + 'static> Node for Pass<T> {
    fn create_runner(self: Box<Self>) -> Box<dyn Runner + Send> {
        Box::new(PassRunner {
            input: self.input.into(),
            output: self.output.into(),
        })
    }
}
