#!/usr/bin/env bash
# XENO-TRL4 TOOLCHAIN PINNING
set -euo pipefail

export RUST_VERSION="stable"
export RUSTUP_TOOLCHAIN="stable"

export LLVM_VERSION="18.1.8"
export LLVM_SYS_181_PREFIX="/opt/llvm-18.1.8"
export MLIR_VERSION="18.1.8"
export MLIR_SYS_181_PREFIX="/opt/mlir-18.1.8"

export Z3_VERSION="4.13.0"
export Z3_PREFIX="/opt/z3-4.13.0"

export CUDA_VERSION="12.4.1"
export ROCM_VERSION="6.1.0"

export LEAN_VERSION="4.9.0"
export COQ_VERSION="8.18.0"
export PYTHON_VERSION="3.12.4"
