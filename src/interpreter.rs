use std::rc::Rc;
use crate::t::{
    ApplyArgs,
    LispType::{self, *},
    List, Func,
};

// use super::t::*;
use super::env::Env;
use super::env::EnvOption;
use super::parse::parse;
use super::procedure::init_procedure;

pub fn eval(exp: &'static str) -> Result<LispType, &str> {
    let root = Env::new();
    init_procedure(&mut root.borrow_mut());
    let env = root.borrow_mut().fork();
    let exp = parse(exp).unwrap();
    let r = interpreter(exp, &mut env.borrow_mut());
    Ok(r)
}

pub fn interpreter(exp: List, env: &mut Env) -> LispType {
    let car = exp.car();
    println!("car: {}", car);
    let cdr = exp.cdr();
    println!("cdr: {} is-exp:{}", cdr, cdr.is_expr());
    match car {
        Symbol(key) => {
            let value = env.get(&key);
            let v = env
                .get(key.as_str())
                .expect(format!("undefined symbol: {}", key).as_str());
            if let Procedure(f) = v.clone() {
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
            if let Procedure(f) = v.clone() {
                if (exp.is_expr()) {
                    println!("exc0: {}", cdr);
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
    f: Rc<Box<dyn Fn(&mut ApplyArgs)->LispType>>,//Func,//fn(&mut ApplyArgs) -> LispType,
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
