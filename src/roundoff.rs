use crate::NumBits;

/// Describes the actual implementation for
/// the basic floating point operations considering
/// the roundoff error.
pub trait RoundoffImpl<M: NumBits>: Clone {
    fn round(&self, f: f64, mantissa_len: &M) -> f64;
    fn add(&self, lhs: f64, rhs: f64, mantissa_len: &M) -> f64 {
        self.round(lhs + rhs, mantissa_len)
    }
    fn sub(&self, lhs: f64, rhs: f64, mantissa_len: &M) -> f64 {
        self.round(lhs - rhs, mantissa_len)
    }
    fn mul(&self, lhs: f64, rhs: f64, mantissa_len: &M) -> f64 {
        self.round(lhs * rhs, mantissa_len)
    }
    fn div(&self, lhs: f64, rhs: f64, mantissa_len: &M) -> f64 {
        self.round(lhs / rhs, mantissa_len)
    }
    fn rem(&self, lhs: f64, rhs: f64, mantissa_len: &M) -> f64 {
        self.round(lhs % rhs, mantissa_len)
    }
}

mod cut; pub use cut::Cut;
mod guard_digit; pub use guard_digit::GuardDigit;
mod random_noise; pub use random_noise::RandomNoise;