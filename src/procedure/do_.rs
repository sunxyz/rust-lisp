use super::*;
use crate::utils::bool_utils::is_true;

fn do_(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.expr();
    let len = list.len();
    if (len < 2) {
        panic!("do: too few arguments");
    }
    let var = list.car();
    let test_then_exp = list.cdr().car();
    let else_body = if (len > 2) {
        Expr(list.cdr().cdr())
    } else {
        Nil
    };
    let apply_args = &mut apply_args.fork_env();
    if let Expr(var0) = var {
        // bind var to val
        var_bind(var0.clone(), apply_args);
        // println!("do: {}", var0);

        if let Expr(l) = test_then_exp {
            let test = l.car();
            let then_body = if l.len() > 1 { Expr(l.cdr()) } else { Nil };
            loop {
                let test = apply_args.inter(&test);
                // println!("do: {} body: {}", test, body);
                if is_true(&test) {
                    return apply_args.inter(&then_body);
                }
                apply_args.inter(&else_body);
                var_update(var0.clone(), apply_args);
            }
        } else {
            panic!("do: test expression must be a list");
        }
    } else {
        panic!("do: variable must be a symbol");
    }
}

fn var_bind(vars: List, apply_args: &mut ApplyArgs) {
    for ele in vars.data() {
        if let Expr(exp) = ele {
            let var = exp.car();
            let val = exp.cdr().car();
            if let Symbol(var0) = var {
                let val = apply_args.inter(&val);
                apply_args.env().ref4write().define(var0.as_str(), val);
            } else {
                panic!("do: variable must be symbol");
            }
        } else {
            panic!("do: variable must be a symbol");
        }
    }
}

fn var_update(vars: List, apply_args: &mut ApplyArgs) {
    for ele in vars.data() {
        if let Expr(exp) = ele {
            if (exp.len() < 2) {
                panic!("do: variable must be update exp");
            }
            let var = exp.car();
            let update_exp = exp.cdr().cdr();
            // println!("update_exp: {}", update_exp);
            // let val = exp.cdr().car();
            if let Symbol(var0) = var {
                let val = apply_args.inter(&Expr(update_exp));
                apply_args.env().ref4write().set(var0.as_str(), val);
                // println! ("{}:{}",var0,apply_args.env().try_lock().expect("locked err").get(var0.as_str()).unwrap());
            } else {
                panic!("do: variable must be symbol");
            }
        } else {
            panic!("do: variable must be a symbol");
        }
    }
}

fn while_fn(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.expr();
    if (list.len() < 2) {
        panic!("while: too few arguments");
    }
    let test_exp = list.car();
    let body = Expr(list.cdr());
    let apply_args = &mut apply_args.fork_env();
    let mut res = Nil;
    loop {
        let test = apply_args.inter(&test_exp);
        if is_true(&test) {
            res=apply_args.inter(&body);
        } else {
            return res;
        }
    }
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("do", do_);
    env.reg_procedure("while", while_fn);
}
