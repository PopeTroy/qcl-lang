use super::target_dialect::TargetDialect;
use anyhow::Result;

pub struct QuantumBackend;

impl TargetDialect for QuantumBackend {
    fn name(&self) -> &'static str { "OpenQASM 3.0 / Quil Target" }
    fn compile_hir(&mut self, _hir_ast: &str) -> Result<Vec<u8>> {
        Ok(b"OPENQASM 3.0;\ninclude \"stdgates.inc\";\nqubit[2] q;\nh q[0];\ncz q[0], q[1];".to_vec())
    }
}
