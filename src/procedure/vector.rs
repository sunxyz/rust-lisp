use super::*;
use std::cell::RefCell;

fn make_vector(apply_args: &mut ApplyArgs) -> LispType {
    let mut vec = Vec::new();
    let list = apply_args.args();
    if (list.len() > 0) {
        let size = get_usize(&list.car()).expect("make-vector: invalid argument");
        let v = if list.len() > 1 {
            list.cdr().car()
        } else {
            Nil
        };
        for i in 0..size {
            vec.push(v.clone());
        }
    } else {
        panic!("make_vector: wrong integer of arguments");
    }
    LispType::vector_of(vec)
}

fn vector(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    let vec = list.data().clone();
    LispType::vector_of(vec)
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
        panic!("vector=?: wrong integer of arguments");
    }
    let arg1 = list.car();
    let arg2 = list.cdr().car();
    Boolean(arg1 == arg2)
}

fn vector_length(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if let Vector(v, s) = list.car() {
        LispType::integer_of(s as isize)
    } else {
        LispType::integer_of(-1)
    }
}

fn vector_ref(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() == 2) {
        if let Vector(l, s) = list.car() {
            let index = list.cdr().car();
            if let Number(NumberBox::Integer(i)) = index {
                if i < 0 {
                    panic!("vector-ref: index out of range");
                }
                if i >= s as isize {
                    panic!("vector-ref: index out of range");
                }
                l.ref4read()[i as usize].clone()
            } else {
                panic!("vector-ref: invalid argument");
            }
        } else {
            panic!("vector-ref: not a vector");
        }
    } else {
        panic!("vector-ref: wrong integer of arguments");
    }
}

fn vector_set(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() == 3) {
        if let Vector(l, s) = list.car() {
            let index = list.cdr().car();
            if let Number(NumberBox::Integer(i)) = index {
                if i < 0 {
                    panic!("vector-set!: index out of range");
                }
                if i >= s as isize {
                    panic!("vector-set!: index out of range");
                }
                l.ref4write()[i as usize] = list.cdr().cdr().car();
                Nil
            } else {
                panic!("vector-set!: invalid argument");
            }
        } else {
            panic!("vector-set!: not a vector");
        }
    } else {
        panic!("vector-set!: wrong integer of arguments");
    }
}

fn vector_fill(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() == 2) {
        if let Vector(l, s) = list.car() {
            let v = list.cdr().car();
            for i in 0..s {
                l.ref4write()[i] = v.clone();
            }
            Nil
        } else {
            panic!("vector-fill!: not a vector");
        }
    } else {
        panic!("vector-fill!: wrong integer of arguments");
    }
}

fn vector2list(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 1) {
        panic!("vector->list: wrong integer of arguments");
    }
    if let Vector(l, s) = list.car() {
        let mut vec = Vec::new();
        for i in 0..s {
            vec.push(l.ref4read()[i].clone());
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
