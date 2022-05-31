use super::*;
use std::collections::HashMap;

fn let_(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.expr();
    let let_x = list.car();
    let body = list.cdr();
    let env = Env::extend(apply_args.env());
    if let Expr(l) = let_x {
        for elem in l.data() {
            if let Expr(kv) = elem {
                if (kv.len() != 2) {
                    panic!("let: invalid argument");
                } else {
                    let key = kv.car();
                    let value = kv.cdr().car().clone();
                    if let Symbol(s) = key {
                        let v = apply_args.inter(&value);
                        env.borrow_mut().define(&s, v);
                    } else {
                        panic!("let: invalid argument");
                    }
                }
            } else {
                panic!("let: invalid argument");
            }
        }
    } else {
        panic!("let: not a list");
    }
    apply_args.inter_4_env(&Expr(body), env)
}

fn let_x(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.expr();
    let let_x = list.car();
    let body = list.cdr();
    let env = Env::extend(apply_args.env());
    if let Expr(l) = let_x {
        for elem in l.data() {
            if let Expr(kv) = elem {
                if (kv.len() != 2) {
                    panic!("let*: invalid argument");
                } else {
                    let key = kv.car();
                    let value = kv.cdr().car().clone();
                    if let Symbol(s) = key {
                        let v = apply_args.inter_4_env(&value, env.clone());
                        env.borrow_mut().define(&s, v);
                    } else {
                        panic!("let*: invalid argument");
                    }
                }
            } else {
                panic!("let*: invalid argument");
            }
        }
    } else {
        panic!("let*: not a list");
    }
    apply_args.inter_4_env(&Expr(body), env)
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("let", let_);
    env.reg_procedure("let*", let_x);
}
