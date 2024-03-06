use std::fmt::Debug;

pub trait NumBits: Clone + Debug {
    fn num_bits(&self) -> u8;
    fn least_bits(&self, other: &Self) -> Self {
        if self.num_bits() < other.num_bits() {
            self.clone()
        } else {
            other.clone()
        }
    }
}

impl NumBits for u8 {
    fn num_bits(&self) -> u8 {
        *self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Bits<const N: u8>;

impl<const N: u8> NumBits for Bits<N> {
    fn num_bits(&self) -> u8 {
        N
    }
}