use super::*;

fn list(apply_args: &mut ApplyArgs) -> LispType {
    Expr(apply_args.args().clone())
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("list", list);
}
