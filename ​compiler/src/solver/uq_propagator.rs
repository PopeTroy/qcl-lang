use nalgebra::DMatrix;

pub struct UncertaintyPropagator;

impl UncertaintyPropagator {
    pub fn propagate_covariance(jacobian: &DMatrix<f64>, cov_in: &DMatrix<f64>) -> DMatrix<f64> {
        jacobian * cov_in * jacobian.transpose()
    }
}
