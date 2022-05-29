use super::*;

fn define_macro(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.expr();
    if (list.data().len() != 2) {
        panic!("define-macro: invalid args");
    }
    if let Symbol(var) = list.car() {
        let proc = list.cdr().car();
        println!("define-macro: {}", var);
        if let Procedure(_) = apply_args.inter(&proc) {
           
            apply_args.env().define(
                &var,
                Procedure(Rc::new(Box::new(move |x| {
                    let mut expr = List::new();
                    let mut args = List::new();
                    args.push_all(vec![Symbol("'".to_string()), Expr(x.expr().clone())]);
                    expr.push_all(vec![Symbol("apply".to_string()), proc.clone(), Expr(args.clone())]);
                    println!("define-macro: {} => {}", args, expr);
                    let template = x.inter(&Expr(expr));
                    x.inter(&template)
                }))),
            );
        } else {
            panic!("define-macro: invalid proc");
        }
    } else {
        panic!("define-macro: invalid var");
    }

    Nil
}

fn render(apply_args: &mut ApplyArgs) -> LispType {
    let t = render0(&apply_args.expr().clone(), apply_args);
    Expr(t)
}

fn render0(exp: &List, apply_args: &mut ApplyArgs) -> List {
    let mut list = List::new();
    for elem in exp.data().clone() {
        println!("elem:{}",elem);
        if let Symbol(k) = elem {
            if let Some(0) = k.find(",@") {
                println!("render: ,@");
                let name = k.clone().replace(",@", "");
                let v = apply_args.inter(&Symbol(name));
                if let Expr(l) = v {
                    list.push_all(l.data().clone());
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
            list.push_all(render0(&l, apply_args).data().clone());
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
