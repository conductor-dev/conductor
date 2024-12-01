use crate::implement_unary_trait_operation;
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

implement_unary_trait_operation!(NormNode, NormRunner, Norm, norm);
