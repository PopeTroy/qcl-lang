use super::target_dialect::TargetDialect;
use anyhow::Result;

pub struct MlirBackend;

impl TargetDialect for MlirBackend {
    fn name(&self) -> &'static str { "MLIR Accelerator Dialect" }
    fn compile_hir(&mut self, _hir_ast: &str) -> Result<Vec<u8>> {
        Ok(b"module { func.func @qcl_kernel() { return } }".to_vec())
    }
}
