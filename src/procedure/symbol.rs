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
    env.reg_procedure("symbol->string", symbol2string);
}
