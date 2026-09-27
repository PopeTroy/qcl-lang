use nalgebra::DMatrix;

pub struct FrameBundle {
    pub base_space_dim: usize,
    pub fiber_dim: usize,
}

impl FrameBundle {
    pub fn parallel_transport(&self, vector: &DMatrix<f64>, connection: &DMatrix<f64>) -> DMatrix<f64> {
        connection * vector
    }
}
