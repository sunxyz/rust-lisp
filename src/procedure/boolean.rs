use super::*;
use crate::utils::bool_utils::is_true;

fn and(apply_args: &mut ApplyArgs) -> LispType {
    // apply_args.apply()
    let mut result = Boolean(false);
    for exp in apply_args.expr().data() {
        let v = apply_args.inter(&exp);
        if is_true(&v) {
            result = v;
        } else {
            return Boolean(false);
        }
    }
    result
}

fn or(apply_args: &mut ApplyArgs) -> LispType {
    for exp in apply_args.expr().data() {
        let v = apply_args.inter(&exp);
        if is_true(&v) {
            return v;
        }
    }
    Boolean(false)
}

fn not(apply_args: &mut ApplyArgs) -> LispType {
    let v = apply_args.inter(&apply_args.expr().car());
    if is_true(&v) {
        Boolean(false)
    } else {
        Boolean(true)
    }
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("and", and);
    env.reg_procedure("or", or);
    env.reg_procedure("not", not);
}
