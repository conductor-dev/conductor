use conductor_core::{
    ports::{InputPort, InputPortCell, OutputPort, OutputPortCell},
    Node, Runner,
};

pub struct PassRunner<T: Clone> {
    input: InputPort<T>,
    output: OutputPort<T>,
}

impl<T: Clone> Runner for PassRunner<T> {
    fn run(self: Box<Self>) {
        loop {
            let value = self.input.recv().unwrap();
            self.output.send(&value);
        }
    }
}

pub struct Pass<T: Clone> {
    pub input: InputPortCell<T>,
    pub output: OutputPortCell<T>,
}

impl<T: Clone> Pass<T> {
    pub fn new() -> Self {
        Self {
            input: InputPortCell::<T>::new(),
            output: OutputPortCell::<T>::new(),
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
