use super::*;

fn apply(apply_args: &mut ApplyArgs) -> LispType {
    apply_args.apply()
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("apply", apply);
}
