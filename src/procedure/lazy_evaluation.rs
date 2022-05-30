use super::*;

fn delay(apply_args: &mut ApplyArgs) -> LispType {
    LispType::cons_of(
        Strings("promise".to_string()),
        Expr(apply_args.expr().clone()),
    )
}

fn is_promise(apply_args: &mut ApplyArgs) -> LispType {
    let args = apply_args.args();
    if args.len() != 1 {
        panic!("is_promise: wrong number of arguments");
    }
    match args.car() {
        Cons(c) => {
            let car = c.borrow().get(0).unwrap().clone();
            if let Strings(s) = car {
                if s == "promise" {
                    return Boolean(true);
                }
            }
        }
        _ => {}
    }
    Boolean(false)
}

fn force(apply_args: &mut ApplyArgs) -> LispType {
    let args = apply_args.args();
    if args.len() != 1 {
        panic!("force: wrong number of arguments");
    }
    let car = args.car();
    if let Cons(c) = car {
        let car = c.borrow().get(0).unwrap().clone();
        if let Strings(s) = car {
            if s == "promise" {
                let cdr = c.borrow().get(1).unwrap().clone();
                if let Expr(list) = cdr {
                   return apply_args.inter(&Expr(list.clone()));
                }
            }
        }
    }
    panic!("force: not a promise");
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("delay", delay);
    env.reg_procedure("promise?", is_promise);
    env.reg_procedure("force", force);
}
