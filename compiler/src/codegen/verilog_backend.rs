use super::target_dialect::TargetDialect;
use anyhow::Result;

pub struct VerilogBackend;

impl TargetDialect for VerilogBackend {
    fn name(&self) -> &'static str { "Verilog RTL Target" }
    fn compile_hir(&mut self, _hir_ast: &str) -> Result<Vec<u8>> {
        Ok(b"module qcl_accel(input clk, output reg ready); endmodule".to_vec())
    }
}
