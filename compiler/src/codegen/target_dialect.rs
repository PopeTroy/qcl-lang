use anyhow::Result;

pub enum CompileTarget {
    LLVM,
    SPIRV,
    MLIR,
    Verilog,
    Quantum,
}

pub trait TargetDialect {
    fn name(&self) -> &'static str;
    fn compile_hir(&mut self, hir_ast: &str) -> Result<Vec<u8>>;
}
