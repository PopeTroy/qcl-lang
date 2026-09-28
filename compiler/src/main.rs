mod parser;
mod hir;
mod solver;
mod codegen;

use std::collections::HashMap;
use anyhow::Result;
use inkwell::context::Context;
use codegen::llvm_backend::LlvmCodegen;
use solver::dim_solver::{DimensionVector, unify_dimensions};

fn main() -> Result<()> {
    println!("Initializing QCL Compiler Kernel v0.1.0...");

    // Test Dimension Unification Kernel
    let length = DimensionVector::base(0);
    let time = DimensionVector::base(2);
    let velocity = length.div(&time);
    
    println!("Validating velocity dimension solver bounds...");
    unify_dimensions(&velocity, &velocity, &HashMap::new())
        .map_err(anyhow::Error::msg)?;
    println!("Dimension solver: SAT.");

    // Test LLVM Codegen Setup
    let context = Context::create();
    let codegen = LlvmCodegen::new(&context, "qcl_core");
    let struct_ty = codegen.get_uncertain_tensor_type();
    
    println!("Generated LLVM Type Structure: StructType({:?})", struct_ty);
    Ok(())
}
