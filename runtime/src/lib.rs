use nalgebra::{SMatrix, SVector};

#[repr(C)]
pub enum DistributionType {
    Gaussian = 0,
    Uniform = 1,
    Poisson = 2,
}

#[repr(C)]
pub struct UncertainVector3 {
    pub value: SVector<f64, 3>,
    pub covariance: SMatrix<f64, 3, 3>,
    pub dist_type: DistributionType,
}

impl UncertainVector3 {
    pub fn new(value: SVector<f64, 3>, covariance: SMatrix<f64, 3, 3>) -> Self {
        Self {
            value,
            covariance,
            dist_type: DistributionType::Gaussian,
        }
    }
}
