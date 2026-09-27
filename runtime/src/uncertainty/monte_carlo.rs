use rand::Rng;

pub struct DistributedMCMC;

impl DistributedMCMC {
    pub fn sample_mean<F>(&self, samples: usize, sampler: F) -> f64
    where F: Fn() -> f64 {
        let mut sum = 0.0;
        for _ in 0..samples {
            sum += sampler();
        }
        sum / (samples as f64)
    }
}
