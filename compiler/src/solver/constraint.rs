use egg::*;

define_language! {
    pub enum PhysicsLanguage {
        Num(i32),
        "+" = Add([Id; 2]),
        "*" = Mul([Id; 2]),
        Symbol(Symbol),
    }
}

pub fn simplify_physics_expr(expr: &str) -> String {
    let parsed: RecExpr<PhysicsLanguage> = expr.parse().unwrap();
    let runner = Runner::default().with_expr(&parsed);
    let root = runner.roots[0];
    let extractor = Extractor::new(&runner.egraph, AstSize);
    let (_best_cost, best) = extractor.find_best(root);
    best.to_string()
}
