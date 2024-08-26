pub mod pipeline;
pub mod ports;
pub mod prelude;

pub trait Runner {
    fn run(self: Box<Self>);
}

pub trait Node {
    fn create_runner(self: Box<Self>) -> Box<dyn Runner + Send>;
}
