use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cut;

impl<M: NumBits> RoundoffImpl<M> for Cut {
    fn add(&self, lhs: f64, rhs: f64, _mantissa_len: &M) -> f64 {
        lhs + rhs
    }
    fn sub(&self, lhs: f64, rhs: f64, mantissa_len: &M) -> f64 {
        RoundoffImpl::<M>::add(self, lhs, -rhs, mantissa_len)
    }
    fn mul(&self, lhs: f64, rhs: f64, _mantissa_len: &M) -> f64 {
        lhs * rhs
    }
    fn div(&self, lhs: f64, rhs: f64, _mantissa_len: &M) -> f64 {
        lhs / rhs
    }
    fn rem(&self, lhs: f64, rhs: f64, _mantissa_len: &M) -> f64 {
        lhs % rhs
    }
}