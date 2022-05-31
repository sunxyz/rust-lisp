use super::*;
use std::cell::RefCell;

fn cons(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if list.len() > 1 {
        let car = list.car();
        let cdr = list.cdr().car();
        Cons(Rc::new(RefCell::new(vec![car, cdr])))
    } else {
        panic!("cons: wrong number of arguments");
    }
}

fn car(apply_args: &mut ApplyArgs) -> LispType {
    let lisp = apply_args.args().car();
    if let Cons(c) = lisp {
        c.borrow().get(0).unwrap().clone()
    } else if let Expr(l) = lisp {
        l.car()
    } else {
        panic!("car: not a cons");
    }
}

fn cdr(apply_args: &mut ApplyArgs) -> LispType {
    let lisp = apply_args.args().car();
    if let Cons(c) = lisp {
        c.borrow().get(1).unwrap().clone()
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
        c.borrow_mut()[0] = car;
        Nil
    } else {
        panic!("car: not a cons");
    }
}

fn set_cdr(apply_args: &mut ApplyArgs) -> LispType {
    let lisp = apply_args.args().car();
    let cdr = apply_args.args().cdr().car();
    if let Cons(c) = lisp {
        c.borrow_mut()[1] = cdr;
        Nil
    } else {
        panic!("cdr: not a cons");
    }
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("cons", cons);
    env.reg_procedure("car", car);
    env.reg_procedure("cdr", cdr);
    env.reg_procedure("set-car!", set_car);
    env.reg_procedure("set-cdr!", set_cdr);
}
