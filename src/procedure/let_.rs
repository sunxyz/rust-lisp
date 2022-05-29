use super::*;
use std::collections::HashMap;

fn let_(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.expr();
    let let_x = list.car();
    let body = list.cdr();
    let mut lex_env = HashMap::new();
    if let Expr(l) = let_x {
        for elem in l.data() {
            if let Expr(kv) = elem {
                if (kv.data().len() != 2) {
                    panic!("let_: invalid argument");
                } else {
                    let key = kv.car();
                    let value = kv.cdr().car().clone();
                    if let Symbol(s) = key {
                        lex_env.insert(s, apply_args.inter(&value));
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
    apply_args.env().fork();
    for (k, v) in lex_env.iter() {
        apply_args.env().define(k, v.clone());
    }
    apply_args.inter(&Expr(body));
    apply_args.env().kill();

    Nil
}

fn let_x(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.expr();
    let let_x = list.car();
    let body = list.cdr();
    apply_args.env().fork();
    if let Expr(l) = let_x {
        for elem in l.data() {
            if let Expr(kv) = elem {
                if (kv.data().len() != 2) {
                    panic!("let_: invalid argument");
                } else {
                    let key = kv.car();
                    let value = kv.cdr().car().clone();
                    if let Symbol(s) = key {
                        let v = apply_args.inter(&value);
                        apply_args.env().define(&s, v);
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
    apply_args.inter(&Expr(body));
    apply_args.env().kill();

    Nil
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("let", let_);
    env.reg_procedure("let*", let_x);
}
