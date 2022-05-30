use super::*;

fn is_procedure(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if list.len() != 1 {
        panic!("wrong number of arguments");
    }
    match list.car() {
        Procedure(_) => Boolean(true),
        _ => Boolean(false),
    }
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("procedure?", is_procedure);
}
