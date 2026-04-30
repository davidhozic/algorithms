//! Shows basic usage of finite difference algorithms
use finite_difference::*;

fn derivable(x1: f64, x2: f64, x3: f64) -> f64 {
    5.0 * x1 + 10.0 * x2 + 3.141592 * x3
}


fn main() {
    /*************************/
    /* First order gradients */
    /*************************/
    // Centered
    let grad_x1 = gradient_centered(
        |args| derivable(args[0], args[1], args[2]),
        &[1.0, 2.0, 3.0], 0, 1e-5
    );
    let grad_x2 = gradient_centered(
        |args| derivable(args[0], args[1], args[2]),
        &[1.0, 2.0, 3.0], 1, 1e-5
    );
    let grad_x3 = gradient_centered(
        |args| derivable(args[0], args[1], args[2]),
        &[1.0, 2.0, 3.0], 2, 1e-5
    );

    println!("Estimated gradients (centered difference): x1={grad_x1}, x2={grad_x2}, x3={grad_x3}");

    // Forward
    let grad_x1 = gradient_forward(
        |args| derivable(args[0], args[1], args[2]),
        &[1.0, 2.0, 3.0], 0, 1e-5
    );
    let grad_x2 = gradient_forward(
        |args| derivable(args[0], args[1], args[2]),
        &[1.0, 2.0, 3.0], 1, 1e-5
    );
    let grad_x3 = gradient_forward(
        |args| derivable(args[0], args[1], args[2]),
        &[1.0, 2.0, 3.0], 2, 1e-5
    );

    println!("Estimated gradients (forward difference):  x1={grad_x1}, x2={grad_x2}, x3={grad_x3}");

    // Backward
    let grad_x1 = gradient_backward(
        |args| derivable(args[0], args[1], args[2]),
        &[1.0, 2.0, 3.0], 0, 1e-5
    );
    let grad_x2 = gradient_backward(
        |args| derivable(args[0], args[1], args[2]),
        &[1.0, 2.0, 3.0], 1, 1e-5
    );
    let grad_x3 = gradient_backward(
        |args| derivable(args[0], args[1], args[2]),
        &[1.0, 2.0, 3.0], 2, 1e-5
    );

    println!("Estimated gradients (backward difference):  x1={grad_x1}, x2={grad_x2}, x3={grad_x3}");
} 
