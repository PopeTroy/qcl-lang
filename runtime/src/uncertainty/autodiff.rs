pub struct ForwardAutoDiff;

impl ForwardAutoDiff {
    pub fn dual_derivative<F>(f: F, x: f64) -> f64
    where F: Fn(f64) -> f64 {
        let eps = 1e-8;
        (f(x + eps) - f(x - eps)) / (2.0 * eps)
    }
}
