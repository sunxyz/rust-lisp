use super::*;

fn is_nil(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.is_nil()) {
        Boolean(true)
    } else {
        if (list.data().len() > 1) {
            panic!("is_nil: too many arguments");
        } else {
            match list.car() {
                Nil => Boolean(true),
                Expr(l) => Boolean(l.is_nil()),
                _ => Boolean(false),
            }
        }
    }
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("nil?", is_nil);
}
