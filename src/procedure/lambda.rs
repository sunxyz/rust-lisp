use super::*;

fn lambda(apply_args: &mut ApplyArgs) -> Box<dyn Fn(&mut ApplyArgs) -> LispType> {
    if let Expr(args) = apply_args.expr().car() {
        let body = Expr(apply_args.expr().cdr());
        Box::new(move |x| {
            x.env().fork();
            println!("----------------");
            bind_args(args.clone(), x.args().clone(), x.env());
            let v = x.inter(&body);
            x.env().kill();
            v
        })
    } else {
        panic!("lambda: args is not list");
    }
}

fn bind_args(args_name: List, args_val: List, env: & mut  Env) {
    println!("bind_args: {} : {}", args_name, args_val);
    let mut next = true;
    let mut args_name = args_name.clone();
    let mut args_val = args_val.clone();
    while next {
        let k = args_name.car();
        let v = args_val.car();
        match k {
            Symbol(name) => {
                if (name == ".") {
                    if (args_name.data().len() == 2) {
                        let key = args_name.cdr().car().clone();
                        if let Symbol(name) = key {
                            env.define(name.as_str(), Expr(args_val.clone()));
                            println!("key:{} v:{}",name,args_val);
                        } else {
                            panic!("lambda: bind_args: key is not symbol");
                        }
                        return;
                    } else {
                        panic!("lambda: wrong number of arguments");
                    }
                } else {
                    println!("{}:{}", name, v);
                    env.define(name.as_str(), v.clone());
                }
            }
            _ => {
                panic!("lambda: args name is not symbol");
            }
        }
        args_name = args_name.cdr();
        args_val = args_val.cdr();
        next = !args_name.is_nil()
    }
}

pub fn reg_procedure(env: &mut Env) {
    let f: fn(&mut ApplyArgs) -> LispType = |apply_args| Procedure(Rc::new(lambda(apply_args)));
    env.reg_procedure("lambda", f);
}
