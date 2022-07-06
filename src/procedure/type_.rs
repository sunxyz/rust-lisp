use super::*;

fn get_type_name(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if list.len() != 1 {
        panic!("get-type-name: wrong integer of arguments");
    }
    let arg = list.car();
    Strings(get_type_name0(&arg))
}

fn is_type(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if list.len() != 2 {
        panic!("is-type?: wrong integer of arguments");
    }
    let arg1 = list.car();
    let arg2 = list.cdr().car();
    if let Symbol(s) = arg2 {
        if get_type_name0(&arg1) == s {
            Boolean(true)
        } else {
            Boolean(false)
        }
    } else {
        panic!("is-type?: not a type name");
    }
}

fn get_type_name0(arg: &LispType) -> String {
    let a = match arg {
        Number(_) => "number",
        Boolean(_) => "boolean",
        Symbol(_) => "symbol",
        Strings(_) => "string",
        Vector(_, _) => "vector",
        Procedure(_) => "procedure",
        Nil => "nil",
        Cons(_) => "cons",
        Expr(_) => "expression",
        Input(_) => "input-port",
        Output(_) => "output-port",
        _ => "unknown",
    };
    a.to_string()
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("get-type-name", get_type_name);
    env.reg_procedure("is-type?", is_type);
}
