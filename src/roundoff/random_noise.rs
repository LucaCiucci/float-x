use super::*;

#[derive(Debug, Clone)]
pub struct RandomNoise;

impl<M: NumBits> RoundoffImpl<M> for RandomNoise {
    fn round(&self, _f: f64, _mantissa_len: &M) -> f64 {
        todo!()
    }
}