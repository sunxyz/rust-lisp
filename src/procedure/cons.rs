use super::*;
use std::cell::RefCell;

fn is_cons(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if list.len() != 1 {
        panic!("is_cons: wrong number of arguments");
    }
    if let Cons(_) = list.car() {
        Boolean(true)
    } else {
        Boolean(false)
    }
}

fn cons_eq(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if list.len() != 2 {
        panic!("cons_eq: wrong number of arguments");
    }
    let car = list.car();
    let cdr = list.cdr();
    if let Cons(c) = car {
        if let Cons(cdr0) = cdr.car() {
            if c.eq(&cdr0) {
                return Boolean(true);
            }
        }
    }
    return Boolean(false);
}

fn cons(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if list.len() == 2 {
        let car = list.car();
        let cdr = list.cdr().car();
        Cons_::new(car, cdr)
    } else {
        panic!("cons: wrong number of arguments");
    }
}

fn car(apply_args: &mut ApplyArgs) -> LispType {
     let list = apply_args.args();
     if(list.len() != 1) {
         panic!("car: wrong number of arguments");
     }
    let lisp = list.car();
    if let Cons(c) = lisp {
       c.car()
    } else if let Expr(l) = lisp {
        l.car()
    } else {
        panic!("car: not a cons");
    }
}

fn cdr(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if(list.len() != 1) {
        panic!("car: wrong number of arguments");
    }
    let lisp = list.car();
    if let Cons(c) = lisp {
        c.cdr()
    } else if let Expr(l) = lisp {
        Expr(l.cdr())
    } else {
        panic!("car: not a cons");
    }
}

fn set_car(apply_args: &mut ApplyArgs) -> LispType {
    let lisp = apply_args.args().car();
    let car = apply_args.args().cdr().car();
    if let Cons(c) = lisp {
        // c.borrow_mut()[0] = car;
        c.set_car(car);
        Nil
    } else {
        panic!("car: not a cons");
    }
}

fn set_cdr(apply_args: &mut ApplyArgs) -> LispType {
    let lisp = apply_args.args().car();
    let cdr = apply_args.args().cdr().car();
    if let Cons(c) = lisp {
        c.set_cdr(cdr);
        Nil
    } else {
        panic!("cdr: not a cons");
    }
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("cons?", is_cons);
    env.reg_procedure("cons=?", cons_eq);
    env.reg_procedure("cons", cons);
    env.reg_procedure("car", car);
    env.reg_procedure("cdr", cdr);
    env.reg_procedure("set-car!", set_car);
    env.reg_procedure("set-cdr!", set_cdr);
}
