use super::*;

fn get_type_name(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if list.len() != 1 {
        panic!("get-type-name: wrong number of arguments");
    }
    let arg = list.car();
    Strings(get_type_name0(&arg))
}

fn is_type(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if list.len() != 2 {
        panic!("is-type?: wrong number of arguments");
    }
    let arg1 = list.car();
    let arg2 = list.cdr().car();
    if let Strings(s) = arg2 {
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
        Symbol(_) => "symbol",
        Strings(_) => "string",
        Number(_) => "number",
        Boolean(_) => "boolean",
        Procedure(_) => "procedure",
        Nil => "nil",
        Cons(_) => "cons",
        Vector(_, _) => "vector",
        Expr(_) => "expr",
        _ => panic!("type: not a type"),
    };
    a.to_string()
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("get-type-name", get_type_name);
    env.reg_procedure("is-type?", is_type);
}
