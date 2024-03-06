use crate::utils::*;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GuardDigit;

impl<M: NumBits> RoundoffImpl<M> for GuardDigit {
    fn round(&self, f: f64, mantissa_len: &M) -> f64 {
        round_last_digit(f, cut_len(mantissa_len.num_bits()))
    }
}