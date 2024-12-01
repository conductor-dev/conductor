use crate::implement_unary_trait_operation;
use conductor_core::{
    ports::{NodeConfigInputPort, NodeConfigOutputPort, NodeRunnerInputPort, NodeRunnerOutputPort},
    NodeConfig, NodeRunner,
};

pub trait Sin {
    type Output;

    fn sin(self) -> Self::Output;
}

impl Sin for f64 {
    type Output = f64;

    fn sin(self) -> Self::Output {
        f64::sin(self)
    }
}

impl Sin for f32 {
    type Output = f32;

    fn sin(self) -> Self::Output {
        f32::sin(self)
    }
}

pub trait Cos {
    type Output;

    fn cos(self) -> Self::Output;
}

impl Cos for f64 {
    type Output = f64;

    fn cos(self) -> Self::Output {
        f64::cos(self)
    }
}

impl Cos for f32 {
    type Output = f32;

    fn cos(self) -> Self::Output {
        f32::cos(self)
    }
}

pub trait Tan {
    type Output;

    fn tan(self) -> Self::Output;
}

impl Tan for f64 {
    type Output = f64;

    fn tan(self) -> Self::Output {
        f64::tan(self)
    }
}

impl Tan for f32 {
    type Output = f32;

    fn tan(self) -> Self::Output {
        f32::tan(self)
    }
}

implement_unary_trait_operation!(Sine, SineRunner, Sin, sin);
implement_unary_trait_operation!(Cosine, CosineRunner, Cos, cos);
implement_unary_trait_operation!(Tangent, TangentRunner, Tan, tan);
