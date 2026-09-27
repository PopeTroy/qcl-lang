use nalgebra::DMatrix;

pub struct CliffordAlgebraSolver {
    pub dimension: usize,
    pub signature_p: usize,
    pub signature_q: usize,
}

impl CliffordAlgebraSolver {
    pub fn new(p: usize, q: usize) -> Self {
        Self {
            dimension: p + q,
            signature_p: p,
            signature_q: q,
        }
    }

    pub fn outer_product(&self, a: &DMatrix<f64>, b: &DMatrix<f64>) -> DMatrix<f64> {
        // Wedge product implementation for exterior algebra
        a * b - b * a
    }
}
