use super::*;

fn define_macro(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.expr();
    if (list.len() != 2) {
        panic!("define-macro: invalid args");
    }
    if let Symbol(var) = list.car() {
        let proc = list.cdr().car();
        // println!("define-macro: {}", var);
        if let Procedure(_) = apply_args.inter(&proc) {
            let proc = Procedure(Rc::new(Box::new(move |x| {
                let mut expr = List::new();
                let mut args = List::new();
                args.push_vec(vec![Symbol("'".to_string()), Expr(x.expr().clone())]);
                expr.push_vec(vec![
                    Symbol("apply".to_string()),
                    proc.clone(),
                    Expr(args.clone()),
                ]);
                // println!("define-macro: {} => {}", args, expr);
                let template = x.inter(&Expr(expr));
                // println!("define-macro template:=> {}", template);
                x.inter(&template)
            })));
            apply_args.env().borrow_mut().define(&var, proc);
        } else {
            panic!("define-macro: invalid proc");
        }
    } else {
        panic!("define-macro: invalid var");
    }

    Nil
}

fn render(apply_args: &mut ApplyArgs) -> LispType {
    if (apply_args.expr().len() != 1) {
        panic!("render: invalid args");
    }
    let car = apply_args.expr().car();
    if let Expr(list) = car {
        let t = render0(&list, apply_args);
        Expr(t)
    } else {
        car
    }
}

fn render0(exp: &List, apply_args: &mut ApplyArgs) -> List {
    let mut list = List::new();
    for elem in exp.clone() {
        // println!("elem:{}", elem);
        if let Symbol(k) = elem {
            if let Some(0) = k.find(",@") {
                // println!("render: ,@");
                let name = k.clone().replace(",@", "");
                let v = apply_args.inter(&Symbol(name));
                if let Expr(l) = v {
                    list.push_all(l);
                } else {
                    list.push(v);
                }
            } else if let Some(0) = k.find(",") {
                let name = k.clone().replace(",", "");
                let v = apply_args.inter(&Symbol(name));
                list.push(v);
            } else {
                list.push(Symbol(k.clone()));
            }
        } else if let Expr(l) = elem {
            list.push(Expr(render0(&l, apply_args)));
        } else {
            list.push(elem);
        }
    }
    list
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("define-macro", define_macro);
    env.reg_procedure("`", render);
}
