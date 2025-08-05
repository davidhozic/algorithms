# optimization-algorithms
My implementation of various optimization algorithms, in Rust.

## Running examples
Each implemented algorithm contains at least one example.
Examples can be run with the command `cargo run --release --example <example>`.

The library used for plotting is [gnuplot](http://www.gnuplot.info/) and needs to be installed
on the system for the examples that draw plots.


## Algorithms implemented
Below is a list of all the algorithms currently implemented in this repository.

### [Kernel Density Estimation (KDE)](./kde)
KDE is a non-parameteric method for estimating the density over a data distribution.
This implementation uses a Gaussian kernel.
