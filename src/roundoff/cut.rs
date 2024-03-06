use crate::utils::*;
use super::*;

/// Drops off the remaining bits of the mantissa.
///
/// This is the simplest form of roundoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cut;

impl<M: NumBits> RoundoffImpl<M> for Cut {
    fn round(&self, f: f64, mantissa_len: &M) -> f64 {
        drop_off(f, cut_len(mantissa_len.num_bits()))
    }
}