use super::target_dialect::TargetDialect;
use anyhow::Result;

pub struct SpirvBackend;

impl TargetDialect for SpirvBackend {
    fn name(&self) -> &'static str { "SPIR-V Compute Shader Target" }
    fn compile_hir(&mut self, _hir_ast: &str) -> Result<Vec<u8>> {
        // Generates SPIR-V binary words
        Ok(vec![0x03, 0x02, 0x23, 0x07])
    }
}
