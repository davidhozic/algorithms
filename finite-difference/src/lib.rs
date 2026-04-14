//! Implements finite difference algorithms:
//! - [`centered`](gradient_centered)
//! - [`forward`](gradient_forward)
//! - [`backward`](gradient_backward)


/// Estimates the partial derivative of `fun` (R^x -> R) using the finite difference method (centered).
/// # Parameters
/// - `fun`: function to derive.
/// - `args`: arguments to `fun`.
/// - `index`: index (of `args`) of the parameter over which to derive `fun`.
/// - `eps`: step size over which to evaluate the gradient.
pub fn gradient_centered(fun: impl Fn(&[f64]) -> f64, args: &[f64], index: usize, eps: f64) -> f64 {
    let arg_diff = args[index];

    let mut args_high = args.to_vec();
    let mut args_low = args.to_vec();

    args_high[index] = arg_diff + eps;
    args_low[index] = arg_diff - eps;
    (fun(&args_high) - fun(&args_low)) / (2.0 * eps)
}

/// Estimates the partial derivative of `fun` (R^x -> R) using the finite difference method (forward).
/// # Parameters
/// - `fun`: function to derive.
/// - `args`: arguments to `fun`.
/// - `index`: index (of `args`) of the parameter over which to derive `fun`.
/// - `eps`: step size over which to evaluate the gradient.
pub fn gradient_forward(fun: impl Fn(&[f64]) -> f64, args: &[f64], index: usize, eps: f64) -> f64 {
    let arg_diff = args[index];

    let mut args_high = args.to_vec();
    args_high[index] = arg_diff + eps;

    (fun(&args_high) - fun(&args)) / eps
}

/// Estimates the partial derivative of `fun` (R^x -> R) using the finite difference method (backward).
/// # Parameters
/// - `fun`: function to derive.
/// - `args`: arguments to `fun`.
/// - `index`: index (of `args`) of the parameter over which to derive `fun`.
/// - `eps`: step size over which to evaluate the gradient.
pub fn gradient_backward(fun: impl Fn(&[f64]) -> f64, args: &[f64], index: usize, eps: f64) -> f64 {
    let arg_diff = args[index];

    let mut args_low = args.to_vec();
    args_low[index] = arg_diff - eps;

    (fun(&args) - fun(&args_low)) / eps
}
