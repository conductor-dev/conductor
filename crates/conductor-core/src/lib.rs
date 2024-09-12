pub mod buffer;
pub mod pipeline;
pub mod ports;
pub mod prelude;
pub mod timer;

pub trait NodeRunner {
    fn run(self: Box<Self>);
}

pub trait NodeConfig {
    fn into_runner(self: Box<Self>) -> Box<dyn NodeRunner + Send>;
}
