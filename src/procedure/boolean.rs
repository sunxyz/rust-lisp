use super::*;

fn and(apply_args: &mut ApplyArgs) -> LispType {
    // apply_args.apply()
    let mut result = Boolean(false);
    for exp in apply_args.expr().data().clone() {
        let v = apply_args.inter(&exp);
        match v {
            Boolean(b) => {
                result = v;
                if !b {
                    return Boolean(false);
                }
            }
            Number(n) => {
                if n != 0 {
                    result = v;
                } else {
                    return Boolean(false);
                }
            }
            String(s) => {
                if s != "" {
                    result = String(s.clone());
                } else {
                    return Boolean(false);
                }
            }
            Char(c) => {
                if c != '\0' {
                    result = v;
                } else {
                    return Boolean(false);
                }
            }
            Nil => {
                return Boolean(false);
            }
            _ => {
                result = v;
            }
        }
    }
    result
}

fn or(apply_args: &mut ApplyArgs) -> LispType {
    // apply_args.apply()
    let mut result = Boolean(false);
    for exp in apply_args.expr().data().clone() {
        let v = apply_args.inter(&exp);
        match v {
            Boolean(b) => {
                if b {
                    return v;
                }
            }
            Number(n) => {
                if n != 0 {
                    return v;
                }
            }
            String(s) => {
                if s != "" {
                    return String(s.clone());
                }
            }
            Char(c) => {
                if c != '\0' {
                    return v;
                }
            }
            Nil => {
                result = Boolean(false);
            }
            _ => {
                return v;
            }
        }
    }
    result
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("and", and);
    env.reg_procedure("or", or);
}
