use super::*;

fn quote(apply_args: &mut ApplyArgs) -> LispType {
    Expr(apply_args.expr().clone())
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("quote", quote);
}
