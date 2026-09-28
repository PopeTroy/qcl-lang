//! Driver module for coordinating compilation steps in `qcl-compiler`.

use anyhow::Result;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct DriverConfig {
    pub input_path: PathBuf,
    pub output_path: Option<PathBuf>,
    pub optimize: bool,
}

pub struct CompilerDriver {
    config: DriverConfig,
}

impl CompilerDriver {
    pub fn new(config: DriverConfig) -> Self {
        Self { config }
    }

    pub fn compile(&self) -> Result<()> {
        tracing::info!("Executing compilation driver for: {:?}", self.config.input_path);
        Ok(())
    }
}
