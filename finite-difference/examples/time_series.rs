//! Shows how to use the finite difference method to calculate a full function derivative.
use std::f64;

use finite_difference::*;

use gnuplot::{self as plt, AxesCommon, PlotOption, Tick};

const TIMESTEP_S: f64 = 0.020;
const N_SAMPLES: usize = (2.1 * f64::consts::PI / 0.020) as usize;
const EPSILON: f64 = 1e-3;

fn main() {
    let time: [_; N_SAMPLES] = std::array::from_fn(|x| x as f64 * TIMESTEP_S );
    let sin_x = time.map(|x| x.sin());

    let cos_x = time.map(
        |x| gradient_backward(|args| f64::sin(args[0]),  // analytical gradient of sin is cos
        &[x], 0, EPSILON
    ));

    let minus_sin_x = time.map(
        |x| gradient_backward(|args| f64::cos(args[0]),  // analytical gradient of cos is -sin
        &[x], 0, EPSILON
    ));

    let mut figure = plt::Figure::new();
    figure.axes2d()
        .set_x_label("Sample", &[])
        .set_y_label("y", &[])
        .lines(&time, &sin_x, &[PlotOption::Caption("sin(x)"), PlotOption::LineStyle(gnuplot::DashType::Dash)])
        .lines(&time, &cos_x, &[PlotOption::Caption("d/dx [sin(x)] -> cos(x)")])
        .lines(&time, &minus_sin_x, &[PlotOption::Caption("d/dx [cos(x)] -> -sin(x)")])
        .set_x_grid(true)
        .set_y_grid(true)
        .set_x_ticks_custom(
        &[
            Tick::Major(0.0, gnuplot::AutoOption::Fix("0")),
            Tick::Major(std::f64::consts::PI / 2.0, gnuplot::AutoOption::Fix("π/2")),
            Tick::Major(std::f64::consts::PI, gnuplot::AutoOption::Fix("π")),
            Tick::Major(3.0 * std::f64::consts::PI / 2.0, gnuplot::AutoOption::Fix("3π/2")),
            Tick::Major(2.0 * std::f64::consts::PI, gnuplot::AutoOption::Fix("2π")),
        ],
        &[],
        &[],
    );

    figure.show().unwrap();
}