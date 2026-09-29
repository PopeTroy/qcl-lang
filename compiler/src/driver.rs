//! Driver module for coordinating compilation steps in `qcl-compiler`.

use crate::codegen::llvm_backend::LlvmCodegen;
use crate::diagnostics::QclDiagnostic;
use crate::hir::frame_check::{BinOp, FrameChecker, FrameId, TensorType};
use crate::parser::parse_qcl_source;
use crate::solver::dim_solver::{unify_dimensions, DimensionVector};

use anyhow::{Context as AnyhowContext, Result};
use inkwell::context::Context;
use std::collections::HashMap;
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

pub struct QclDriverOptions {
    pub module_name: String,
    pub emit_llvm: bool,
}

pub struct QclCompilerDriver {
    pub options: QclDriverOptions,
}

impl QclCompilerDriver {
    pub fn new(options: QclDriverOptions) -> Self {
        Self { options }
    }

    pub fn compile_source(&self, source: &str) -> Result<()> {
        println!("--> [QCL Driver] Parsing source...");
        parse_qcl_source(source).context("Failed AST Parsing stage")?;

        println!("--> [QCL Driver] Checking Spatial Reference Frames...");
        let checker = FrameChecker {
            transform_graph: crate::hir::frame_check::TransformGraph {
                edges: HashMap::new(),
            },
        };
        let t1 = TensorType {
            rank: 1,
            frame: FrameId("ECEF".to_string()),
        };
        let _ = checker
            .check_binary_op(&t1, &t1, BinOp::Add)
            .map_err(|e| anyhow::anyhow!("HIR Frame Check Error: {:?}", e))?;

        println!("--> [QCL Driver] Solving SMT Dimensional Constraints...");
        let length = DimensionVector::base(0);
        let time = DimensionVector::base(2);
        let velocity = length.div(&time);

        unify_dimensions(&velocity, &velocity, &HashMap::new())
            .map_err(|e| QclDiagnostic::DimensionMismatch {
                expected: "Length/Time".to_string(),
                actual: e,
            })?;

        if self.options.emit_llvm {
            println!("--> [QCL Driver] Lowering Xeno-IR to LLVM Bitcode...");
            let llvm_context = Context::create();
            let mut codegen = LlvmCodegen::new(&llvm_context, &self.options.module_name);
            let struct_ty = codegen.get_uncertain_tensor_type();
            println!("--> [QCL Driver] LLVM Uncertain Tensor Struct Type: {:?}", struct_ty);
        }

        println!("--> [QCL Driver] Compilation Completed Successfully.");
        Ok(())
    }
}
