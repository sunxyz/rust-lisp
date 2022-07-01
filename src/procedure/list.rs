use super::*;
use crate::utils::bool_utils::is_true;
use std::cell::RefCell;

fn is_list(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 1) {
        panic!("list?: wrong number of arguments");
    }
    let arg = list.car();
    if let Expr(l) = arg {
        Boolean(true)
    } else {
        Boolean(false)
    }
}

fn list(apply_args: &mut ApplyArgs) -> LispType {
    Expr(apply_args.args().clone())
}

fn list_eq(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 2) {
        panic!("list=?: wrong number of arguments");
    }
    let arg1 = list.car();
    let arg2 = list.cdr().car();
    Boolean(arg1 == arg2)
}

fn list_length(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 1) {
        panic!("list-length: wrong number of arguments");
    }
    let arg = list.car();
    if let Expr(l) = arg {
        Number(l.len() as isize)
    } else {
        panic!("list-length: not a list");
    }
}

fn list_ref(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 2) {
        panic!("list-ref: wrong number of arguments");
    }
    let arg = list.car();
    if let Expr(l) = arg {
        let index = list.cdr().car();
        if let Number(i) = index {
            if i < 0 {
                panic!("list-ref: index out of range");
            }
            if i >= l.len() as isize {
                panic!("list-ref: index out of range");
            }
            l.data()[i as usize].clone()
        } else {
            panic!("list-ref: not a number");
        }
    } else {
        panic!("list-ref: not a list");
    }
}

fn list_tail(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 2) {
        panic!("list-tail: wrong number of arguments");
    }
    let arg = list.car();
    if let Expr(l) = arg {
        let index = list.cdr().car();
        if let Number(i) = index {
            if i < 0 {
                panic!("list-tail: index out of range");
            }
            if i >= l.len() as isize {
                panic!("list-tail: index out of range");
            }
            Expr(List::of(l.data()[i as usize..].to_vec()))
        } else {
            panic!("list-tail: not a number");
        }
    } else {
        panic!("list-tail: not a list");
    }
}

fn list_set(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 3) {
        panic!("list-set!: wrong number of arguments");
    }
    let arg = list.car();
    if let Expr(l) = arg {
        let index = list.cdr().car();
        if let Number(i) = index {
            if i < 0 {
                panic!("list-set!: index out of range");
            }
            if i >= l.len() as isize {
                panic!("list-set!: index out of range");
            }
            l.data()[i as usize] = list.cdr().cdr().car().clone();
            Expr(l)
        } else {
            panic!("list-set!: not a number");
        }
    } else {
        panic!("list-set!: not a list");
    }
}

fn append(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() < 1) {
        panic!("append: wrong number of arguments");
    }
    let result = list.car().clone();
    let args = list.cdr();
    if let Expr(r) = result {
        let mut r = r.clone();
        args.data().iter().for_each(|arg| {
            if let Expr(l) = arg {
                r.push_all(l.clone());
            } else {
                panic!("append: not a list");
            }
        });
    } else {
        panic!("append: not a list");
    }
    Nil
}

fn add(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() < 1) {
        panic!("append: wrong number of arguments");
    }
    let result = list.car().clone();
    let args = list.cdr();
    if let Expr(r) = result {
        let mut r = r.clone();
        args.data().iter().for_each(|arg| {
            r.push(arg.clone());
        });
    } else {
        panic!("append: not a list");
    }
    Nil
}

fn reverse(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 1) {
        panic!("reverse: wrong number of arguments");
    }
    let arg = list.car();
    if let Expr(l) = arg {
        let t = l.data().clone().into_iter().rev().collect();
        Expr(List::of(t))
    } else {
        panic!("reverse: not a list");
    }
}

fn list2vector(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 1) {
        panic!("list->vector: wrong number of arguments");
    }
    let arg = list.car();
    if let Expr(l) = arg {
       LispType::vector_of(l.data())
    } else {
        panic!("list->vector: not a list");
    }
}

fn list2string(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 1) {
        panic!("list->string: wrong number of arguments");
    }
    let arg = list.car();
    if let Expr(l) = arg {
        Strings(
            l.data()
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(""),
        )
    } else {
        panic!("list->string: not a list");
    }
}

fn map(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() < 2) {
        panic!("map: wrong number of arguments");
    }
    let proc = list.car();
    let v = list.cdr().car();
    let mut result = List::new();
    if let Expr(l) = v {
        if (l.len() == 0) {
            return Expr(result);
        }
        let last = get_last(list);
        if let Procedure(proc) = proc {
            for i in 0..l.len() {
                let elem = l.data()[i].clone();
                let mut args = vec![elem];
                for o in last.iter() {
                    args.push(o.get(i).or(Some(&Nil)).unwrap().clone());
                }
                result.push(proc.read()(&mut apply_args.clone_of(Some(List::of(args)))));
            }
        } else {
            panic!("map: not a procedure");
        }
    } else {
        panic!("map: not a list");
    }
    Expr(result)
}

fn filter(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 2) {
        panic!("filter: wrong number of arguments");
    }
    let proc = list.car();
    let args = list.cdr().car();
    let mut result = List::new();
    if let Procedure(proc) = proc {
        if let Expr(l) = args {
            if (l.len() == 0) {
                return Expr(result);
            } else {
                for elem in l.data() {
                    if is_true(&proc.read()(
                        &mut apply_args.clone_of(Some(List::of(vec![elem.clone()]))),
                    )) {
                        result.push(elem.clone());
                    }
                }
            }
        } else {
            panic!("filter: not a list");
        }
    } else {
        panic!("filter: not a procedure");
    }
    Expr(result)
}

fn reduce(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 3) {
        panic!("reduce: wrong number of arguments");
    }
    let proc = list.car();
    let mut result = list.cdr().car();
    let args = list.cdr().cdr().car();
    if let Procedure(proc) = proc {
        if let Expr(l) = args {
            if (l.len() == 0) {
                return result.clone();
            } else {
                for elem in l.data() {
                    result = proc.read()(
                        &mut apply_args
                            .clone_of(Some(List::of(vec![result.clone(), elem.clone()]))),
                    );
                }
            }
        } else {
            panic!("reduce: not a list");
        }
    } else {
        panic!("reduce: not a procedure");
    }
    result
}

fn for_each(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() < 2) {
        panic!("for-each: wrong number of arguments");
    }
    let proc = list.car();
    let v = list.cdr().car();
    if let Expr(l) = v {
        if (l.len() == 0) {
            return Nil;
        }
        let last = get_last(list);
        if let Procedure(proc) = proc {
            for i in 0..l.len() {
                let elem = l.data()[i].clone();
                let mut args = vec![elem];
                for o in last.iter() {
                    args.push(o.get(i).or(Some(&Nil)).unwrap().clone());
                }
                proc.read()(&mut apply_args.clone_of(Some(List::of(args))));
            }
        } else {
            panic!("map: not a procedure");
        }
    } else {
        panic!("for-each: not a list");
    }
    Nil
}

fn get_last(list: &List) -> Vec<Vec<LispType>> {
    if (list.data().len() > 2) {
        list.cdr()
            .cdr()
            .data()
            .iter()
            .map(|x| {
                if let Expr(l) = x {
                    l.data().clone()
                } else {
                    Vec::new()
                }
            })
            .collect::<Vec<Vec<LispType>>>()
    } else {
        Vec::new()
    }
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("list?", is_list);
    env.reg_procedure("list=?", list_eq);
    env.reg_procedure("list", list);
    env.reg_procedure("list-ref", list_ref);
    env.reg_procedure("list-tail", list_tail);
    env.reg_procedure("list-set!", list_set);
    env.reg_procedure("list-length", list_length);
    env.reg_procedure("list-add", add);
    env.reg_procedure("append", append);
    env.reg_procedure("reverse", reverse);
    env.reg_procedure("list->vector", list2vector);
    env.reg_procedure("list->string", list2string);
    env.reg_procedure("map", map);
    env.reg_procedure("for-each", for_each);
    env.reg_procedure("filter", filter);
    env.reg_procedure("reduce", reduce);
}
