//! Gaussian Kernel Density Estimation Example
//! This fits and plots a Gaussian density function to the data.

use kde::KernelDensity;

use rand_distr::{Normal, Distribution};
use gnuplot::{self as plt, AxesCommon};
use rand;



fn main() {
    /* Generate some data */
    let mut rng = rand::rng();
    // The data will be a joint distribution of 2 Gaussians with means of -10 and 10 with std of 5,
    // so two hills are expected in the density.
    let normal_1 = Normal::new(10.0, 5.0).unwrap();
    let normal_2 = Normal::new(-10.0, 5.0).unwrap();
    let mut data_to_fit: Vec<_> = (0..100).map(|_| normal_1.sample(&mut rng)).collect();
    data_to_fit.extend((0..100).map(|_| normal_2.sample(&mut rng)));

    /* Estimate density */
    let mut kde = KernelDensity::new();
    kde.fit(&data_to_fit);

    /* Plot */
    let input: Vec<_> = (-2000..2000).map(|x| x as f64 / 100.0).collect();
    let y: Vec<_> = input.iter().map(|x| kde.predict(*x)).collect();

    let mut figure = plt::Figure::new();
    figure.axes2d()
        .set_title("Estimated density", &[])
        .set_x_label("value", &[])
        .set_y_label("density [count]", &[])
        .set_x_grid(true)
        .set_y_grid(true)
        .lines(input, y, &[]);

    figure.show().unwrap();
}
