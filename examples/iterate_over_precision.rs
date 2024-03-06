use std::iter::once;

use float_x::*;

fn main() {
    let pi_f64 = std::f64::consts::PI;
    let pi_f32 = std::f32::consts::PI;

    // garbage, don't look at these lines, just building
    // a nice iterator over the example precision values
    let lengths = (0..=22)
        .step_by(2)
        .chain(once(23))
        .chain((24..=52).step_by(4));

    for mantissa_len in lengths {
        let pi = FloatX::new(
            pi_f64,
            mantissa_len,
            roundoff::GuardDigit,
        );
        let pi2 = pi * pi;
        println!("pi^2 ({mantissa_len:02} bits mantissa, {} precision): {pi2:21.18}", precision_name(mantissa_len));
    }

    println!("pi^2 (f64)                             : {:21.18}", pi_f64 * pi_f64);
    println!("pi^2 (f32)                             : {:21.18}", pi_f32 * pi_f32);
}

fn precision_name(mantissa_len: u8) -> &'static str {
    match mantissa_len {
        10 => " f16",
        23 => " f32",
        52 => " f64",
        _ => "N.A."
    }
}