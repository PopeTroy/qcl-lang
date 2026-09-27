use z3::{Context, Solver, Ast, Int, Config, SatResult};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TypeError {
    #[error("Dimension mismatch error")]
    DimensionMismatch,
    #[error("Solver error")]
    SolverFailure,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DimensionVector {
    // [L, M, T, I, Θ, N, J]
    pub exponents: [i32; 7],
}

impl DimensionVector {
    pub fn new() -> Self {
        Self { exponents: [0; 7] }
    }

    pub fn base(idx: usize) -> Self {
        let mut v = Self::new();
        if idx < 7 {
            v.exponents[idx] = 1;
        }
        v
    }

    pub fn mul(&self, other: &Self) -> Self {
        Self {
            exponents: std::array::from_fn(|i| self.exponents[i] + other.exponents[i]),
        }
    }

    pub fn div(&self, other: &Self) -> Self {
        Self {
            exponents: std::array::from_fn(|i| self.exponents[i] - other.exponents[i]),
        }
    }

    pub fn pow(&self, n: i32) -> Self {
        Self {
            exponents: std::array::from_fn(|i| self.exponents[i] * n),
        }
    }

    pub fn assert_eq_z3<'ctx>(&self, solver: &Solver<'ctx>, ctx: &'ctx Context, vars: &[Int<'ctx>]) {
        for (i, &e) in self.exponents.iter().enumerate() {
            let target = Int::from_i64(ctx, e as i64);
            solver.assert(&vars[i]._eq(&target));
        }
    }
}

pub fn unify_dimensions(
    expected: &DimensionVector,
    actual: &DimensionVector,
) -> Result<(), TypeError> {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let expected_vars: Vec<Int> = (0..7)
        .map(|i| Int::new_const(&ctx, format!("exp_{}", i)))
        .collect();

    expected.assert_eq_z3(&solver, &ctx, &expected_vars);

    for (i, &act_exp) in actual.exponents.iter().enumerate() {
        let act_val = Int::from_i64(&ctx, act_exp as i64);
        solver.assert(&expected_vars[i]._eq(&act_val));
    }

    if solver.check() == SatResult::Sat {
        Ok(())
    } else {
        Err(TypeError::DimensionMismatch)
    }
}
