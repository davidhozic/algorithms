//! Kernel Density Estimation (KDE)
//! Here we implement the kernel density estimation algorithm
//! with an example of the Gaussian kernel.

use crate::kernels::gaussian_kernel;

pub mod kernels;

pub struct KernelDensity<'a> {
    bandwidth: Option<f64>,
    u_data: &'a[f64]
}

impl<'a> KernelDensity<'a> {
    pub fn new() -> Self {
        Self {bandwidth: None, u_data: &[]}
    }

    pub fn with_bandwidth(bandwidth: f64) -> Self {
        Self{bandwidth: Some(bandwidth), u_data: &[]}
    }

    /* Public interface */
    pub fn fit(&mut self, u: &'a[f64]) {
        self.u_data = u;
    }

    pub fn predict(&self, u: f64) -> f64 {
        let mut total = 0.0;
        let mut dist;
        let bandwidth = self.bandwidth.unwrap_or_else(|| self.silverman_bandwidth());

        for x in self.u_data {
            dist = (u - x) / bandwidth;
            total += gaussian_kernel(dist);
        }
        total
    }

    /* Private methods */
    fn silverman_bandwidth(&self) -> f64 {
        let mean = self.u_data.iter().sum::<f64>() / self.u_data.len() as f64;
        let std = (self.u_data.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (self.u_data.len() - 1) as f64).sqrt();
        1.06 * std * ((self.u_data.len() as f64).powf(-0.2))
    }
}
