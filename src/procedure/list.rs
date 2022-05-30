use super::*;
use crate::utils::bool_utils::is_true;

fn list(apply_args: &mut ApplyArgs) -> LispType {
    Expr(apply_args.args().clone())
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
                result.push(proc(&mut apply_args.clone_of(Some(List::of(args)))));
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
                for elem in l {
                    if is_true(&proc(
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
                for elem in l{
                    result = proc(
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
                proc(&mut apply_args.clone_of(Some(List::of(args))));
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
    env.reg_procedure("list", list);
    env.reg_procedure("map", map);
    env.reg_procedure("for-each", for_each);
}
