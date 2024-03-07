use std::{io::Write, ops::{AddAssign, Div, Mul}};

use float_x::{roundoff, FloatX};



fn main() {
    let n = 2u64.pow(22);

    let start = std::time::Instant::now();
    print!("computing pi with f32... ");
    std::io::stdout().flush().unwrap();
    let pi_f32 = leibniz_pi(n, |f| f as f32);
    println!("done in {:?}", start.elapsed());
    let start = std::time::Instant::now();
    print!("computing pi with f64... ");
    std::io::stdout().flush().unwrap();
    let pi_f64 = leibniz_pi(n, |f| f as f64);
    println!("done in {:?}", start.elapsed());
    println!("f32: {:.16}", pi_f32);
    println!("f64: {:.16}", pi_f64);

    let mut pi_fx = Vec::new();
    let mut pi_fx_cut = Vec::new();
    for mantissa_len in 0..=52 {
        let pi = leibniz_pi(n, |f| FloatX::new(f as _, mantissa_len, roundoff::GuardDigit));
        pi_fx.push(pi.repr());
        println!("{}: {:.16}", mantissa_len, pi.repr());
        let pi = leibniz_pi(n, |f| FloatX::new(f as _, mantissa_len, roundoff::Truncate));
        pi_fx_cut.push(pi.repr());
        println!("{} - trunc: {:.16}", mantissa_len, pi.repr());
        plot(&pi_fx, &pi_fx_cut);
    }
}

fn leibniz_pi<T>(
    n: u64,
    f: impl Fn(i64) -> T,
) -> T
where
    T: AddAssign + Div<Output = T> + Mul<Output = T>,
{
    let mut pi: T = f(0);
    for i in 0..n {
        let sign = if i % 2 == 0 { 1 } else { -1 };
        let term = f(sign) / f(2 * i as i64 + 1);
        pi += term;
    }
    pi * f(4)
}

fn plot(pi_fx: &[f64], pi_fx_cut: &[f64]) {
    use plotters::prelude::*;
    let root = SVGBackend::new("examples/leibniz/comparison.svg", (480, 320)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    let diffs = pi_fx.iter()
        .map(|p| (p - std::f64::consts::PI).abs())
        .collect::<Vec<_>>();

    let diffs_cut = pi_fx_cut.iter()
        .map(|p| (p - std::f64::consts::PI).abs())
        .collect::<Vec<_>>();

    let (min, max) = diffs
        .iter()
        .chain(diffs_cut.iter())
        .fold(
            (f64::INFINITY, f64::NEG_INFINITY),
            |(min, max), &d| (min.min(d), max.max(d))
        );

    let mut chart = ChartBuilder::on(&root)
        .caption("Difference with π vs mantissa len", ("Arial", 15).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(60)
        .build_cartesian_2d(0.0..52.0, (min..max).log_scale())
        .unwrap();

    chart
        .configure_mesh()
        .max_light_lines(5)
        .x_desc("Mantissa len [bits]")
        .y_desc("|π - leibniz|")
        .draw()
        .unwrap();

    chart
        .draw_series(LineSeries::new(
            (0..=52).map(|d| d as f64).zip(diffs.iter().cloned()),
            RED.stroke_width(2),
        ))
        .unwrap()
        .label("Guard digit")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED.stroke_width(2)));

    chart
        .draw_series(LineSeries::new(
            (0..=52).map(|d| d as f64).zip(diffs_cut.iter().cloned()),
            BLUE.stroke_width(2),
        ))
        .unwrap()
        .label("Truncate")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE.stroke_width(2)));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .position(SeriesLabelPosition::UpperRight)
        .draw()
        .unwrap();
}