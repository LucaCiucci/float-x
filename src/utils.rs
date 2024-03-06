
pub const REPR_MANTISSA_LEN: u8 = 52;
pub const F64_MANTISSA_MASK: u64 = 0x000fffffffffffff;
pub const F64_NOT_MANTISSA_MASK: u64 = !F64_MANTISSA_MASK;

/// Reinterpret a `f64` as a `u64`
pub fn like_u64(f: f64) -> u64 {
    unsafe { std::mem::transmute(f) }
}

/// Reinterpret a `u64` as a `f64`
pub fn like_f64(u: u64) -> f64 {
    unsafe { std::mem::transmute(u) }
}

pub fn mantissa_from_f64(f: f64) -> u64 {
    like_u64(f) & F64_MANTISSA_MASK
}