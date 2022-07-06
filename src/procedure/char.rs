use super::*;

fn is_char(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if list.len() != 1 {
        panic!("wrong integer of arguments");
    }
    match list.car() {
        Char(_) => Boolean(true),
        _ => Boolean(false),
    }
}

fn char_eq(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 2) {
        panic!("char=?: wrong integer of arguments");
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

fn char2integer(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 1) {
        panic!("char->integer: wrong integer of arguments");
    }
    let arg = list.car();
    if let Char(c) = arg {
        LispType::integer_of(c as isize)
    } else {
        panic!("char->integer: not a char");
    }
}

fn char_up_case(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 1) {
        panic!("char-upcase: wrong integer of arguments");
    }
    let arg = list.car();
    if let Char(c) = arg {
        Char(c.to_ascii_uppercase())
    } else {
        panic!("char-upcase: not a char");
    }
}

fn char_down_case(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 1) {
        panic!("char-downcase: wrong integer of arguments");
    }
    let arg = list.car();
    if let Char(c) = arg {
        Char(c.to_ascii_lowercase())
    } else {
        panic!("char-downcase: not a char");
    }
}

fn digit_value(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 1) {
        panic!("digit-value: wrong integer of arguments");
    }
    let arg = list.car();
    if let Char(c) = arg {
        if c.is_digit(10) {
            LispType::integer_of(c as isize - '0' as isize)
        } else {
            panic!("digit-value: not a digit");
        }
    } else {
        panic!("digit-value: not a char");
    }
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("char?", is_char);
    env.reg_procedure("char=?", char_eq);
    env.reg_procedure("char->integer", char2integer);
    env.reg_procedure("char-upcase", char_up_case);
    env.reg_procedure("char-downcase", char_down_case);
    env.reg_procedure("digit-value", digit_value);
}
