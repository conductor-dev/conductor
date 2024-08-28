use conductor_core::{
    ports::{NodeConfigInputPort, NodeConfigOutputPort, NodeRunnerInputPort, NodeRunnerOutputPort},
    NodeConfig, NodeRunner,
};
pub use conductor_macros::Norm;

pub trait Norm {
    type Output;

    fn norm(self) -> Self::Output;
}

macro_rules! impl_norm {
    ($t:ty) => {
        impl Norm for $t {
            type Output = $t;

            fn norm(self) -> $t {
                self.abs()
            }
        }
    };
}

macro_rules! impl_norm_unsigned {
    ($t:ty) => {
        impl Norm for $t {
            type Output = $t;

            fn norm(self) -> $t {
                self
            }
        }
    };
}

impl_norm!(i8);
impl_norm!(i16);
impl_norm!(i32);
impl_norm!(i64);
impl_norm!(i128);
impl_norm!(isize);

impl_norm!(f32);
impl_norm!(f64);

impl_norm_unsigned!(u8);
impl_norm_unsigned!(u16);
impl_norm_unsigned!(u32);
impl_norm_unsigned!(u64);
impl_norm_unsigned!(u128);
impl_norm_unsigned!(usize);

impl<T> Norm for Option<T>
where
    T: Norm,
{
    type Output = Option<T::Output>;

    fn norm(self) -> Self::Output {
        self.map(Norm::norm)
    }
}

pub struct NormerRunner<O: Clone, T: Norm<Output = O>> {
    input: NodeRunnerInputPort<T>,
    output: NodeRunnerOutputPort<O>,
}

impl<O, T> NodeRunner for NormerRunner<O, T>
where
    O: Clone,
    T: Norm<Output = O> + Clone,
{
    fn run(self: Box<Self>) {
        loop {
            let value = self.input.recv().unwrap();

            let norm = value.norm();

            self.output.send(&norm);
        }
    }
}

pub struct Normer<O: Clone, T: Norm<Output = O>> {
    pub input: NodeConfigInputPort<T>,
    pub output: NodeConfigOutputPort<O>,
}

impl<O: Clone, T: Norm<Output = O>> Normer<O, T> {
    pub fn new() -> Self {
        Self {
            input: NodeConfigInputPort::new(),
            output: NodeConfigOutputPort::new(),
        }
    }
}

impl<O: Clone, T: Norm<Output = O>> Default for Normer<O, T> {
    fn default() -> Self {
        Self::new()
    }
}

// TODO: Can + Send + 'static be removed?
impl<O, T> NodeConfig for Normer<O, T>
where
    O: Clone + Send + 'static,
    T: Norm<Output = O> + Clone + Send + 'static,
{
    fn into_runner(self: Box<Self>) -> Box<dyn NodeRunner + Send> {
        Box::new(NormerRunner {
            input: self.input.into(),
            output: self.output.into(),
        })
    }
}
