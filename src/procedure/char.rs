use super::*;

fn is_char(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if list.len() != 1 {
        panic!("wrong number of arguments");
    }
    match list.car() {
        Char(_) => Boolean(true),
        _ => Boolean(false),
    }
}

fn char_eq(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 2) {
        panic!("char=?: wrong number of arguments");
    }
    let arg1 = list.car();
    let arg2 = list.cdr().car();
    if let Char(c1) = arg1 {
        if let Char(c2) = arg2 {
            Boolean(c1 == c2)
        } else {
            panic!("char=?: not a char");
        }
    } else {
        panic!("char=?: not a char");
    }
}

fn char2number(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 1) {
        panic!("char->number: wrong number of arguments");
    }
    let arg = list.car();
    if let Char(c) = arg {
        Number(c as isize)
    } else {
        panic!("char->number: not a char");
    }
}
pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("char?", is_char);
    env.reg_procedure("char=?", char_eq);
    env.reg_procedure("char->number", char2number);
}
