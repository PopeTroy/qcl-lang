use anyhow::Result;

pub struct PersistentMemoryBuffer {
    pub pool_path: String,
}

impl PersistentMemoryBuffer {
    pub fn persist_state(&self, data: &[u8]) -> Result<()> {
        std::fs::write(&self.pool_path, data)?;
        Ok(())
    }
}
