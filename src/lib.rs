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
    pub fn mantissa_raw_len(&self) -> &M {
        &self.mantissa_len
    }

    pub fn mantissa_len(&self) -> u8 {
        self.mantissa_len.num_bits()
    }

    /// Number of bits to cut off the mantissa
    fn cut_len(&self) -> u8 {
        REPR_MANTISSA_LEN - self.mantissa_len.num_bits()
    }

    /// Mask to cut off the mantissa
    ///
    /// As an example, if the mantissa has length 44 (52 - 8), the mask will be
    /// `0x00000000000000ff` because the last 8 bits cut off the mantissa.
    fn cut_mask(&self) -> u64 {
        (1u64 << self.cut_len()) - 1
    }

    /// Mask to keep
    ///
    /// This is just the bitwise negation of the [cut mask](Self::cut_mask).
    fn keep_mask(&self) -> u64 {
        !self.cut_mask()
    }

    pub fn new(repr: f64, mantissa_len: M, roundoff_impl: Impl) -> Self {
        let mut f = Self { repr, mantissa_len, roundoff_impl };
        debug_assert!(f.mantissa_len() <= REPR_MANTISSA_LEN);
        f.repr = like_f64(like_u64(f.repr) & f.keep_mask());
        f
    }

    pub fn mantissa_raw(&self) -> u64 {
        like_u64(self.repr) & !self.cut_mask()
    }

    pub fn mantissa_shifted(&self) -> u64 {
        self.mantissa_raw() >> self.cut_len()
    }

    pub fn set_mantissa_shifted(&mut self, mantissa: u64) {
        assert!(mantissa & !self.keep_mask() == 0, "Mantissa too large");
        let mantissa = mantissa << self.cut_len();
        self.repr = like_f64((like_u64(self.repr)) | (mantissa & self.keep_mask()));
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