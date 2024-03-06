use std::fmt::{self, Debug, Display};

mod num_bits; pub use num_bits::*;
pub mod roundoff; use roundoff::RoundoffImpl;
pub mod utils; use utils::*;
mod ops;

/// Float with settable number of mantissa digits
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct FloatX<M: NumBits, Impl: RoundoffImpl<M>> {
    repr: f64, // TODO use a trait to allow other types
    mantissa_len: M,
    roundoff_impl: Impl,
}

impl<M: NumBits, Impl: RoundoffImpl<M>> FloatX<M, Impl> {
    /// Rounds off the given float to the given number of mantissa digits
    pub fn new(repr: f64, mantissa_len: M, roundoff_impl: Impl) -> Self {
        debug_assert!(mantissa_len.num_bits() <= REPR_MANTISSA_LEN);
        Self {
            repr: roundoff_impl.round(repr, &mantissa_len),
            mantissa_len,
            roundoff_impl,
        }
    }

    pub fn repr(&self) -> f64 {
        self.repr
    }

    pub fn mantissa_raw_len(&self) -> &M {
        &self.mantissa_len
    }

    pub fn mantissa_len(&self) -> u8 {
        self.mantissa_len.num_bits()
    }

    pub fn mantissa_raw(&self) -> u64 {
        like_u64(self.repr) & !cut_mask(cut_len(self.mantissa_len()))
    }

    pub fn mantissa_shifted(&self) -> u64 {
        self.mantissa_raw() >> cut_len(self.mantissa_len())
    }

    pub fn set_mantissa_shifted(&mut self, mantissa: u64) {
        assert!(mantissa & !keep_mask(cut_len(self.mantissa_len())) == 0, "Mantissa too large");
        let mantissa = mantissa << cut_len(self.mantissa_len());
        self.repr = like_f64((like_u64(self.repr)) | (mantissa & keep_mask(cut_len(self.mantissa_len()))));
    }

    pub fn set_mantissa_propagated(&mut self, _mantissa: u64) {
        todo!()
    }
}

impl<M: NumBits, Impl: RoundoffImpl<M>> Display for FloatX<M, Impl> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.repr, f)
    }
}