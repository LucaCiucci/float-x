
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

pub fn raw_mantissa_from_f64(f: f64) -> u64 {
    like_u64(f) & F64_MANTISSA_MASK
}

pub fn drop_off(f: f64, cut_len: u8) -> f64 {
    like_f64(like_u64(f) & keep_mask(cut_len))
}

pub fn round_last_digit(repr: f64, cut_len: u8) -> f64 {
    if cut_len == 0 {
        return repr;
    }
    let cut = drop_off(repr, cut_len);
    let cut_minus_1 = drop_off(repr, cut_len - 1);
    if like_u64(cut_minus_1) & (1 << (cut_len - 1)) != 0 {
        let no_mantissa = like_f64(like_u64(cut_minus_1) & F64_NOT_MANTISSA_MASK);
        let with_round_mantissa = like_f64((like_u64(cut) & F64_NOT_MANTISSA_MASK) | (1 << cut_len));
        let correction = with_round_mantissa - no_mantissa;
        let result = cut + correction;
        debug_assert!(like_u64(result) & cut_mask(cut_len) == 0);
        result
    } else {
        cut
    }
}

/// Number of bits to cut off the mantissa
pub fn cut_len(mantissa_len: u8) -> u8 {
    REPR_MANTISSA_LEN - mantissa_len
}

/// Mask to cut off the mantissa
///
/// As an example, if the mantissa has length 44 (52 - 8), the mask will be
/// `0x00000000000000ff` because the last 8 bits cut off the mantissa.
pub fn cut_mask(cut_len: u8) -> u64 {
    (1u64 << cut_len) - 1
}

/// Mask to keep
///
/// This is just the bitwise negation of the [cut mask](Self::cut_mask).
pub fn keep_mask(cut_len: u8) -> u64 {
    !cut_mask(cut_len)
}