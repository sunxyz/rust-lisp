use super::*;
use std::cell::RefCell;

fn make_vector(apply_args: &mut ApplyArgs) -> LispType {
    let mut vec = Vec::new();
    let list = apply_args.args();
    let mut size = 0;
    if (list.len() == 2) {
        if let Number(s) = list.car() {
            size = s.abs();
            let v = list.cdr().car();
            for i in 0..size {
                vec.push(v.clone());
            }
        } else {
            panic!("make-vector: invalid argument");
        }
    } else if (list.len() == 1) {
        if let Number(s) = list.car() {
            size = s.abs();
        } else {
            panic!("make-vector: invalid argument");
        }
    } else {
        panic!("make_vector: wrong number of arguments");
    }
    Vector(Rc::new(RefCell::new(vec)), size as usize)
}

fn vector(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    let vec = list.data().clone();
    let size = vec.len();
    Vector(Rc::new(RefCell::new(vec)), size)
}

fn is_vector(apply_args: &mut ApplyArgs) -> LispType {
    if let Vector(_, _) = apply_args.args().car() {
        Boolean(true)
    } else {
        Boolean(false)
    }
}

fn vector_eq(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 2) {
        panic!("vector=?: wrong number of arguments");
    }
    let arg1 = list.car();
    let arg2 = list.cdr().car();
    Boolean(arg1 == arg2)
}

fn vector_length(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if let Vector(v, s) = list.car() {
        Number(s as isize)
    }else {
        Number(-1)
    }
}

fn vector_ref(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() == 2) {
        if let Vector(l, s) = list.car() {
            let index = list.cdr().car();
            if let Number(i) = index {
                if i < 0 {
                    panic!("vector-ref: index out of range");
                }
                if i >= s as isize {
                    panic!("vector-ref: index out of range");
                }
                l.borrow()[i as usize].clone()
            } else {
                panic!("vector-ref: invalid argument");
            }
        } else {
            panic!("vector-ref: not a vector");
        }
    } else {
        panic!("vector-ref: wrong number of arguments");
    }
}

fn vector_set(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() == 3) {
        if let Vector(l, s) = list.car() {
            let index = list.cdr().car();
            if let Number(i) = index {
                if i < 0 {
                    panic!("vector-set!: index out of range");
                }
                if i >= s as isize {
                    panic!("vector-set!: index out of range");
                }
                l.borrow_mut()[i as usize] = list.cdr().cdr().car();
                Nil
            } else {
                panic!("vector-set!: invalid argument");
            }
        } else {
            panic!("vector-set!: not a vector");
        }
    } else {
        panic!("vector-set!: wrong number of arguments");
    }
}

fn vector_fill(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() == 2) {
        if let Vector(l, s) = list.car() {
            let v = list.cdr().car();
            for i in 0..s {
                l.borrow_mut()[i] = v.clone();
            }
            Nil
        } else {
            panic!("vector-fill!: not a vector");
        }
    } else {
        panic!("vector-fill!: wrong number of arguments");
    }
}

fn vector2list(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 1) {
        panic!("vector->list: wrong number of arguments");
    }
    if let Vector(l, s) = list.car() {
        let mut vec = Vec::new();
        for i in 0..s {
            vec.push(l.borrow()[i].clone());
        }
        Expr(List::of(vec))
    } else {
        panic!("vector->list: not a vector");
    }
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("vector?", is_vector);
    env.reg_procedure("vector=?", vector_eq);
    env.reg_procedure("make-vector", make_vector);
    env.reg_procedure("vector", vector);
    env.reg_procedure("vector-length", vector_length);
    env.reg_procedure("vector-ref", vector_ref);
    env.reg_procedure("vector-set!", vector_set);
    env.reg_procedure("vector-fill!", vector_fill);
    env.reg_procedure("vector->list", vector2list);
}
