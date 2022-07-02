use super::*;

fn lambda(apply_args: &mut ApplyArgs, lazy_eval: bool) -> Box<dyn Fn(&mut ApplyArgs) -> LispType + Sync + Send> {
    if let Expr(args_name) = apply_args.expr().car() {
        let body = Expr(apply_args.expr().cdr());
        let e = apply_args.env();
        Box::new(move |x| {
            let env = Env::extend(e.clone());
            if (!args_name.is_nil()) {
                bind_args(
                    args_name.clone(),
                    if lazy_eval {
                        x.expr().clone()
                    } else {
                        x.args().clone()
                    },
                    env.clone(),
                );
            }
            x.inter_4_env(&body, env)
        })
    } else {
        panic!("lambda: args is not list");
    }
}

fn bind_args(args_name: List, args_val: List, env: RefEnv) {
    // println!("bind_args: {} : {}", args_name, args_val);
    let mut next = true;
    let mut args_name = args_name.clone();
    let mut args_val = args_val.clone();
    while next {
        let k = args_name.car();
        let v = args_val.car();
        match k {
            Symbol(name) => {
                if (name == ".") {
                    if (args_name.len() == 2) {
                        let key = args_name.cdr().car().clone();
                        if let Symbol(name) = key {
                            env.ref4write().define(name.as_str(), Expr(args_val.clone()));
                            // println!("key:{} v:{}", name, args_val);
                        } else {
                            panic!("lambda: bind_args: key is not symbol");
                        }
                        return;
                    } else {
                        panic!("lambda: wrong number of arguments");
                    }
                } else {
                    // println!("{}:{}", name, v);
                    env.ref4write().define(name.as_str(), v.clone());
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
    let f: fn(&mut ApplyArgs) -> LispType = |apply_args| LispType::procedure_of(lambda(apply_args, false));
    let lazy_f: fn(&mut ApplyArgs) -> LispType = |apply_args| LispType::procedure_of(lambda(apply_args, false));
    env.reg_procedure("lambda", f);
    env.reg_procedure("lambda-lep", lazy_f);
}
