use float_x::*;

fn main() {
    let pi_f64 = std::f64::consts::PI;

    let pi = FloatX::new(
        pi_f64,
        Fixed::<10>,
        roundoff::GuardDigit,
    );

    let pi2 = pi * pi;
    println!("pi^2 (10 bits mantissa): {pi2:.18}");

    println!("pi^2 (f64)             : {:.18}", pi_f64 * pi_f64);
}