pub struct SurfaceCodeRuntime {
    pub distance: usize,
}

impl SurfaceCodeRuntime {
    pub fn decode_syndrome(&self, syndromes: &[bool]) -> Vec<usize> {
        syndromes.iter().enumerate().filter(|(_, &s)| s).map(|(i, _)| i).collect()
    }
}
