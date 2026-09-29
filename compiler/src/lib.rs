//! # Quantum Continuum Language (QCL) Compiler & Xeno-IR Engine
//!
//! High-performance, physics-aware compiler monorepo enforcing compile-time
//! dimensional vector unification via Z3 SMT, spatial reference frame tracking,
//! and first-class measurement uncertainty tensors lowering to LLVM IR / Cranelift.
//!
//! *Note: This engine is distinct from legacy C++ Quantum Computation Language (Ömer, 1998-2006).*

pub mod driver;
pub mod diagnostics;
pub mod parser;
pub mod hir;
pub mod solver;
pub mod codegen;
