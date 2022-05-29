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
        if (l.data().len() == 0) {
            return Expr(result);
        }
        let last = if (list.data().len() > 2) {
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
        };
        if let Procedure(proc) = proc {
            for i in 0..l.data().len() {
                let elem = l.data()[i].clone();
                let mut args = vec![elem];
                for o in last.iter() {
                    args.push(o.get(i).or(Some(&Nil)).unwrap().clone());
                }
                args.push(Number(i as i32));
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

fn for_each(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    let proc = list.car();
    let v = list.cdr().car();
    if let Expr(l) = v {
        if let Procedure(proc) = proc {
            for i in 0..l.data().len() {
                let elem = l.data()[i].clone();
                proc(&mut apply_args.clone_of(Some(List::of(vec![elem, Number(i as i32)]))));
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
