use nalgebra::SMatrix;

pub struct MetricTensor4D {
    pub g: SMatrix<f64, 4, 4>,
}

impl MetricTensor4D {
    pub fn minkowski() -> Self {
        let mut g = SMatrix::<f64, 4, 4>::zeros();
        g[(0, 0)] = -1.0;
        g[(1, 1)] = 1.0;
        g[(2, 2)] = 1.0;
        g[(3, 3)] = 1.0;
        Self { g }
    }
}
