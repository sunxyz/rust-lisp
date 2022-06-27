use crate::utils::bool_utils::is_true;

use super::*;

fn delay(apply_args: &mut ApplyArgs) -> LispType {
    let env = apply_args.env();
    let expr = apply_args.expr().clone();
    LispType::cons_of(
        LispType::cons_of(
            Symbol("promise".to_string()),
            LispType::procedure_of(Box::new(move |x| {
                // println!("promise: {}", "called");
                x.inter_4_env(&Expr(expr.clone()), env.clone())
            })),
        ),
        LispType::cons_of(Boolean(false), Nil),
    )
}

fn is_promise(apply_args: &mut ApplyArgs) -> LispType {
    let args = apply_args.args();
    Boolean(is_promise0(args))
}

fn is_promise0(args: &List) -> bool {
    if args.len() != 1 {
        panic!("is_promise: wrong number of arguments");
    }
    match args.car() {
        Cons(c) => {
            let car = c.car();
            if let Cons(cons) = car {
                if let Symbol(symbol) = cons.car() {
                    if symbol == "promise" {
                        return true;
                    }
                }
            }
        }
        _ => {}
    }
    false
}

fn force(apply_args: &mut ApplyArgs) -> LispType {
    let args = apply_args.args();
    if args.len() != 1 {
        panic!("force: wrong number of arguments");
    }
    
    if  is_promise0(args){
        let args = args.car();
        if let Cons(c) = args {
            let cdr = c.cdr();
            if let Cons(result) = cdr {
                if is_true(&result.car()) {
                    return result.cdr();
                }else {
                    let car = c.car();
                    if let Cons(cons) = car {
                        if let Procedure(proc) = cons.cdr() {
                           let v = proc.try_read().expect("locked err")(apply_args);
                        //    println!("force: {}", v);
                           result.set_cdr(v.clone());
                           result.set_car(Boolean(true));
                           return v;
                        }
                    }
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
