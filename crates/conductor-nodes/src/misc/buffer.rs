use conductor_core::{
    buffer::CircularBuffer,
    ports::{NodeConfigInputPort, NodeConfigOutputPort, NodeRunnerInputPort, NodeRunnerOutputPort},
    receive, NodeConfig, NodeRunner,
};

struct BufferRunner<T: Clone, S: Into<usize>> {
    send_when_not_full: bool,

    input: NodeRunnerInputPort<T>,
    size: NodeRunnerInputPort<S>,

    output: NodeRunnerOutputPort<Vec<T>>,
}

impl<T: Clone, S: Into<usize>> NodeRunner for BufferRunner<T, S> {
    fn run(self: Box<Self>) {
        let size = self.size.recv();

        let mut buffer = CircularBuffer::new(size.into());

        loop {
            receive! {
                (self.size): size => {
                    buffer.resize(size.into());
                },
                (self.input): value => {
                    buffer.push(value);
                }
            }

            if self.send_when_not_full || buffer.len() == buffer.size() {
                self.output.send(&buffer.clone().into());
            }
        }
    }
}

pub struct Buffer<T: Clone, S: Into<usize>> {
    send_when_not_full: bool,

    pub input: NodeConfigInputPort<T>,
    pub size: NodeConfigInputPort<S>,

    pub output: NodeConfigOutputPort<Vec<T>>,
}

impl<T: Clone, S: Into<usize>> Buffer<T, S> {
    pub fn new(send_when_not_full: bool) -> Self {
        Self {
            send_when_not_full,

            input: NodeConfigInputPort::new(),
            size: NodeConfigInputPort::new(),

            output: NodeConfigOutputPort::new(),
        }
    }
}

impl<T: Clone, S: Into<usize>> Default for Buffer<T, S> {
    fn default() -> Self {
        Self::new(true)
    }
}

// TODO: Can + Send + 'static be removed?
impl<T: Send + 'static + Clone, S: Send + 'static + Into<usize>> NodeConfig for Buffer<T, S> {
    fn into_runner(self: Box<Self>) -> Box<dyn NodeRunner + Send> {
        Box::new(BufferRunner {
            send_when_not_full: self.send_when_not_full,

            input: self.input.into(),
            size: self.size.into(),

            output: self.output.into(),
        })
    }
}
