use crate::{
    env::{Env, EnvOps, RefEnv},
    parser::parser,
    procedure::init_procedure,
    types::{
        ApplyArgs,
        LispType::{self, *},
        List, ProcedureBox, RefOps,
    },
};

pub fn eval(exp: &str) -> Result<LispType, &str> {
    let root = Env::root();
    // let v = root.write();
    init_procedure(&mut root.ref4write());
    let env = Env::extend(root);
    let exp = parser(exp.to_string()).expect("parser error");
    Ok(interpreter(exp, env))
}

pub fn interpreter(exp: List, env: RefEnv) -> LispType {
    let car = exp.car();
    // println!("car: {}", car);
    let cdr = exp.cdr();
    // println!("cdr: {} is-exp:{}", cdr, cdr.is_expr());
    match car {
        Symbol(key) => {
            let v = env
                .ref4read()
                .get(key.as_str())
                .expect(format!("undefined symbol: {}", key).as_str());
            if let Procedure(f) = v.clone() {
                if (exp.is_expr()) {
                    // println!("exc: {}", cdr);
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
            let v = interpreter(l, env.clone());
            if let Procedure(f) = v.clone() {
                if (exp.is_expr()) {
                    // println!("exc0: {}", cdr);
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
    f: ProcedureBox, //Func,//fn(&mut ApplyArgs) -> LispType,
    cdr: List,
    env: RefEnv,
    args: Option<List>,
) -> LispType {
    let lazy_args_f: fn(List, RefEnv) -> List = |exp, e| {
        let mut t = List::new();
        for l in exp.data() {
            t.push(interpreter0(&l, e.clone()));
        }
        t
    };

    f.ref4read()(&mut ApplyArgs::new(
        cdr,
        None,
        lazy_args_f,
        interpreter0,
        env,
    ))
}

fn interpreter0(o: &LispType, env: RefEnv) -> LispType {
    match o {
        Expr(l) => interpreter(l.clone(), env),
        Symbol(s) => env
            .ref4read()
            .get(s.as_str())
            .expect(format!("undefined symbol {}", s.as_str()).as_str()),
        _ => o.clone(),
    }
}
