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
            let proc = LispType::procedure_of(Box::new(move |x| {
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
            }));
            apply_args.env().write().define(&var, proc);
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

fn render0(expr: &List, apply_args: &mut ApplyArgs) -> List {
    let mut list = List::new();
    let data = expr.data();
    let mut pre = Nil;
    let render_single_symbol = Symbol(",".to_string());
    let render_flat_symbol = Symbol(",@".to_string());
    for elem in data {
        // println!("pre : {} elem: {}", pre, elem);
        if render_single_symbol.eq(&pre) {
            reader_single(&mut list, &elem, apply_args);
        } else if render_flat_symbol.eq(&pre) {
            reader_flat(&mut list, &elem, apply_args);
        } else if !(render_single_symbol.eq(&elem) || render_flat_symbol.eq(&elem)) {
            if let Expr(exp) = elem.clone() {
                if(exp.is_nil()){
                    list.push(Expr(exp));
                }else{
                    let r = render0(&exp, apply_args);
                    let car = exp.car();
                    if render_single_symbol.eq(&car) {
                        list.push_all(r);
                    } else if render_flat_symbol.eq(&car){
                        list.push_all(r);
                    }else{
                        list.push(Expr(r));
                    }
                }
            } else {
                list.push(elem.clone())
            }
        }
        pre = elem;
    }
    list
}

fn reader_flat(exps: &mut List, exp: &LispType, apply_args: &mut ApplyArgs) {
    let r = apply_args.inter(exp);
    if let Expr(el) = r {
        exps.push_all(el)
    } else {
        exps.push(r)
    }
}

fn reader_single(exps: &mut List, exp: &LispType, apply_args: &mut ApplyArgs) {
    let r = apply_args.inter(exp);
    exps.push(r)
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("define-macro", define_macro);
    env.reg_procedure("`", render);
}
