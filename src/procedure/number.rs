use super::*;

fn is_number(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 1) {
        panic!("number?: wrong number of arguments");
    }
    let arg = list.car();
    if let Number(_) = arg {
        Boolean(true)
    } else {
        Boolean(false)
    }
}

fn number_eq(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 2) {
        panic!("number=?: wrong number of arguments");
    }
    let arg1 = list.car();
    let arg2 = list.cdr().car();
    if let Number(n1) = arg1 {
        if let Number(n2) = arg2 {
            Boolean(n1 == n2)
        } else {
            panic!("number=?: not a number");
        }
    } else {
        panic!("number=?: not a number");
    }
}

fn number2string(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 1) {
        panic!("number->string: wrong number of arguments");
    }
    let arg = list.car();
    if let Number(n) = arg {
        Strings(n.to_string())
    } else {
        panic!("number->string: not a number");
    }
}

fn number2char(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 1) {
        panic!("number->char: wrong number of arguments");
    }
    let arg = list.car();
    if let Number(n) = arg {
        Char(n as u8 as char)
    } else {
        panic!("number->char: not a number");
    }
}

fn add(apply_args: &mut ApplyArgs) -> LispType {
    calc(apply_args, |a, b| a + b)
}

fn subtract(apply_args: &mut ApplyArgs) -> LispType {
    calc(apply_args, |a, b| a - b)
}

fn multiply(apply_args: &mut ApplyArgs) -> LispType {
    calc(apply_args, |a, b| a * b)
}

fn divide(apply_args: &mut ApplyArgs) -> LispType {
    calc(apply_args, |a, b| a / b)
}

fn calc(apply_args: &mut ApplyArgs, f: fn(i32, i32) -> i32) -> LispType {
    Number(
        apply_args
            .args()
            .clone()
            .map(|x| -> i32 {
                match x {
                    LispType::Number(i) => i,
                    v => panic!("{} not a number", v),
                }
            })
            .reduce(f)
            .unwrap(),
    )
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("number?", is_number);
    env.reg_procedure("number=?", number_eq);
    env.reg_procedure("number->string", number2string);
    env.reg_procedure("number->char", number2char);
    env.reg_procedure("+", add);
    env.reg_procedure("-", subtract);
    env.reg_procedure("*", multiply);
    env.reg_procedure("/", divide);
}
