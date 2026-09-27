#[derive(Debug, Clone)]
pub struct PolynomialChaosExpansion {
    pub order: usize,
    pub coefficients: Vec<f64>,
}

#[derive(Debug, Clone)]
pub struct IntervalBounds {
    pub min: f64,
    pub max: f64,
}

impl IntervalBounds {
    pub fn add(&self, other: &Self) -> Self {
        Self {
            min: self.min + other.min,
            max: self.max + other.max,
        }
    }
}
