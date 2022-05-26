use crate::t::{
    ApplyArgs,
    LispType::{self, *},
    List,
};
// use super::t::*;
use super::env::Env;
use super::parse::parse;
use super::procedure::init_procedure;

pub fn eval(exp: &'static str) -> Result<LispType, &str> {
    let mut root = Env::new();
    init_procedure(&mut root);
    // let mut env = Env::new();
    // // env.set_parent(&'static mut root);
    let exp = parse(exp).unwrap();
    Ok(interpreter(exp, &mut root))
}

pub fn interpreter(exp: List, env: &mut Env) -> LispType {
    let car = exp.car();
    println!("car: {}", car);
    let cdr = exp.cdr();
    println!("cdr: {}", cdr);
    match car {
        Symbol(key) => {
           
            let v = env
                .get(key.as_str())
                .expect(format!("undefined symbol: {}", key).as_str());
            if let Procedure(f) = v {
                if (exp.is_expr()) {
                    println!("exc: {}", cdr);
                    return apply(f, cdr, env, None);
                }
            }
            if (cdr.is_nil()) {
                v
            } else {
                interpreter(cdr, env)
            }
        }
        Expr(l) => {
            let v = interpreter(l, env);
            if let Procedure(f) = v {
                if (exp.is_expr()) {
                    println!("exc: {}", cdr);
                    return apply(f, cdr, env, None);
                }
            }
            if (cdr.is_nil()) {
                v
            } else {
                interpreter(cdr, env)
            }
        }
        _ => {
            if (cdr.is_nil()) {
                car
            } else {
                interpreter(cdr, env)
            }
        }
    }
}

fn apply(
    f: fn(&mut ApplyArgs) -> LispType,
    cdr: List,
    env: &mut Env,
    args: Option<List>,
) -> LispType {
    let lazy_args_f: fn(List, &mut Env) -> List = |exp, e| {
        let mut t = List::new();
        for l in exp.data() {
            t.push(interpreter0(l, e));
        }
        t
    };

    f(&mut ApplyArgs::new(
        cdr,
        None,
        lazy_args_f,
        interpreter0,
        env,
    ))
}

fn interpreter0(o: &LispType, env: &mut Env) -> LispType {
    match o {
        Expr(l) => interpreter(l.clone(), env),
        Symbol(s) => env.get(s.as_str()).unwrap(),
        _ => o.clone(),
    }
}
