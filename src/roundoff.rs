use crate::NumBits;

/// Describes the actual implementation for
/// the basic floating point operations considering
/// the roundoff error.
pub trait RoundoffImpl<M: NumBits>: Clone {
    fn add(&self, lhs: f64, rhs: f64, mantissa_len: &M) -> f64;
    fn sub(&self, lhs: f64, rhs: f64, mantissa_len: &M) -> f64;
    fn mul(&self, lhs: f64, rhs: f64, mantissa_len: &M) -> f64;
    fn div(&self, lhs: f64, rhs: f64, mantissa_len: &M) -> f64;
    fn rem(&self, lhs: f64, rhs: f64, mantissa_len: &M) -> f64;
}

mod cut; pub use cut::Cut;