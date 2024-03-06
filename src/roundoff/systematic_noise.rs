use super::*;

#[derive(Debug, Clone)]
pub struct SystematicNoise;

impl<M: NumBits> RoundoffImpl<M> for SystematicNoise {
    fn round(&self, _f: f64, _mantissa_len: &M) -> f64 {
        todo!()
    }
}