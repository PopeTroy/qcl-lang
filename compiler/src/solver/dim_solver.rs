use z3::{Context, Solver, SatResult, ast::{Ast, Int}};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DimensionVector {
    // [L, M, T, I, Theta, N, J]
    pub exponents: [i32; 7],
}

impl DimensionVector {
    pub fn new() -> Self {
        Self { exponents: [0; 7] }
    }

    pub fn base(idx: usize) -> Self {
        let mut v = Self::new();
        v.exponents[idx] = 1;
        v
    }

    pub fn mul(&self, other: &Self) -> Self {
        Self { exponents: std::array::from_fn(|i| self.exponents[i] + other.exponents[i]) }
    }

    pub fn div(&self, other: &Self) -> Self {
        Self { exponents: std::array::from_fn(|i| self.exponents[i] - other.exponents[i]) }
    }

    pub fn pow(&self, n: i32) -> Self {
        Self { exponents: std::array::from_fn(|i| self.exponents[i] * n) }
    }

    pub fn to_z3_int_vector<'ctx>(&self, ctx: &'ctx Context, prefix: &str) -> Vec<Int> {
        self.exponents.iter().enumerate().map(|(i, _)| {
            Int::new_const(ctx, format!("{}_{}", prefix, i))
        }).collect()
    }

    pub fn assert_eq_z3(&self, solver: &Solver, ctx: &Context, vars: &[Int]) {
        for (i, &e) in self.exponents.iter().enumerate() {
            solver.assert(&vars[i]._eq(&Int::from_i64(ctx, e as i64)));
        }
    }
}

pub fn unify_dimensions(
    expected: &DimensionVector,
    actual: &DimensionVector,
    _generics: &HashMap<String, DimensionVector>,
) -> Result<HashMap<String, DimensionVector>, String> {
    let cfg = z3::Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let exp_vars = expected.to_z3_int_vector(&ctx, "expected");
    let act_vars = actual.to_z3_int_vector(&ctx, "actual");

    expected.assert_eq_z3(&solver, &ctx, &exp_vars);
    actual.assert_eq_z3(&solver, &ctx, &act_vars);

    for i in 0..7 {
        solver.assert(&exp_vars[i]._eq(&act_vars[i]));
    }

    if solver.check() == SatResult::Sat {
        Ok(HashMap::new())
    } else {
        Err(format!("TypeError: Dimension mismatch. Expected: {:?}, Actual: {:?}", expected, actual))
    }
}
