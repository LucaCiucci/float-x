use std::cell::RefCell;

use crate::{cut_len, cut_mask, mantissa_shifted, round_last_digit, set_mantissa_shifted};

use super::*;
use rand::{Rng, SeedableRng};

thread_local! {
    // the simples rng
    static RNG: RefCell<rand::rngs::SmallRng> = RefCell::new(rand::rngs::SmallRng::from_entropy());
}

#[derive(Debug, Clone)]
pub struct RandomNoise;

impl RandomNoise {
    pub fn seed(
        seed: u64,
    ) {
        RNG.with(|rng| {
            *rng.borrow_mut() = rand::rngs::SmallRng::seed_from_u64(seed);
        });
    }
}

impl<M: NumBits> RoundoffImpl<M> for RandomNoise {
    fn round(&self, mut f: f64, mantissa_len: &M) -> f64 {
        // generate 2 bit random noise
        let cut_len = cut_len(mantissa_len.num_bits());
        f = round_last_digit(f, cut_len);
        if mantissa_len.num_bits() > 2 {
            let noise = RNG.with(|rng| rng.borrow_mut().gen::<u8>()) >> 6;
            let new_mantissa = mantissa_shifted(f, cut_len) & cut_mask(cut_len) | noise as u64;
            set_mantissa_shifted(&mut f, cut_len, new_mantissa);
        }
        f
    }
}