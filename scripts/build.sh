#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/toolchain_pins.sh"

echo "=== Building QCL Compiler & Runtime (LLVM ${LLVM_VERSION}, Z3 ${Z3_VERSION}) ==="

cargo build --workspace --release

echo "=== Build Complete ==="
