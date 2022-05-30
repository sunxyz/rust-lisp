use super::*;

fn is_symbol(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 1) {
        panic!("symbol?: wrong number of arguments");
    }
    let arg = list.car();
    if let Symbol(s) = arg {
        Boolean(true)
    } else {
        Boolean(false)
    }
}

fn symbol_eq(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 2) {
        panic!("symbol=?: wrong number of arguments");
    }
    let arg1 = list.car();
    let arg2 = list.cdr().car();
    if let Symbol(s1) = arg1 {
        if let Symbol(s2) = arg2 {
            Boolean(s1 == s2)
        } else {
            panic!("symbol=?: not a symbol");
        }
    } else {
        panic!("symbol=?: not a symbol");
    }
}

fn symbol2string(apply_args: &mut ApplyArgs)->LispType{
    let list = apply_args.args();
    if (list.len() != 1) {
        panic!("symbol->string: wrong number of arguments");
    }
    let arg = list.car();
    if let Symbol(s) = arg {
        Strings(s.to_string())
    } else {
        panic!("symbol->string: not a symbol");
    }
}



pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("symbol?", is_symbol);
    env.reg_procedure("symbol=?", symbol_eq);
    env.reg_procedure("symbol->string", symbol2string);
}
