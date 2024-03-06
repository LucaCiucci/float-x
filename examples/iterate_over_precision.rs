use float_x::{roundoff::Cut, FloatX};

fn main() {
    let pi_f64 = std::f64::consts::PI;

    for mantissa_len in (0..=52).step_by(2) {
        let pi = FloatX::new(pi_f64, mantissa_len, Cut);
        let pi2 = pi * pi;
        println!("pi^2 ({mantissa_len:02} bits mantissa): {pi2:.18} ({pi2:?})");
    }

    println!("pi^2 (f64)             : {:.18}", pi_f64 * pi_f64);
}