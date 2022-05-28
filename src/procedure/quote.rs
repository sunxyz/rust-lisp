use super::*;

fn quote(apply_args: &mut ApplyArgs) -> LispType {
    apply_args.expr().car()
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("quote", quote);
    env.reg_procedure("'", quote)
}
