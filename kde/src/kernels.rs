//! Defines the kernel functions used in KDE.
use std::f64::consts::PI;


/// The Gaussian kernel in the form 
/// 1 / sqrt(2*pi) * exp(-1/2 x^2).
pub fn gaussian_kernel(x: f64) -> f64 {
    (-0.5 * x * x).exp() / (2.0 * PI).sqrt()
}
