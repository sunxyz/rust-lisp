use super::*;

fn list(apply_args: &mut ApplyArgs) -> LispType {
    Expr(apply_args.args().clone())
}

fn map(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    let proc = list.car();
    let v = list.cdr().car();
    let mut result = List::new();
    if let Expr(l) = v {
        if let Procedure(proc) = proc {
            for i in 0..l.data().len() {
                let elem = l.data()[i].clone();
                let mut apply_args0 = ApplyArgs::new(
                    List::new(),
                    Some(List::of(vec![elem, Number(i as i32)])),
                    |l, v| List::new(),
                    apply_args.get_inter(),
                    apply_args.env(),
                );
                result.push(proc(&mut apply_args0));
            }
        } else {
            panic!("map: not a procedure");
        }
    } else {
        panic!("map: not a list");
    }
    Expr(result)
}

fn for_each(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    let proc = list.car();
    let v = list.cdr().car();
    if let Expr(l) = v {
        if let Procedure(proc) = proc {
            for i in 0..l.data().len() {
                let elem = l.data()[i].clone();
                let mut apply_args0 = ApplyArgs::new(
                    List::new(),
                    Some(List::of(vec![elem, Number(i as i32)])),
                    |l, v| List::new(),
                    apply_args.get_inter(),
                    apply_args.env(),
                );
                proc(&mut apply_args0);
            }
        } else {
            panic!("for-each: not a procedure");
        }
    } else {
        panic!("for-each: not a list");
    }
    Nil
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("list", list);
    env.reg_procedure("map", map);
    env.reg_procedure("for-each", for_each);
}
