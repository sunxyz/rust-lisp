use super::*;

const ZERO: NumberBox = NumberBox::Integer(0);
const ONE: NumberBox = NumberBox::Integer(1);

fn is_integer(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 1) {
        panic!("integer?: wrong integer of arguments");
    }
    let arg = list.car();
    if let Number(NumberBox::Integer(_)) = arg {
        Boolean(true)
    } else {
        Boolean(false)
    }
}

fn is_float(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 1) {
        panic!("float?: wrong float of arguments");
    }
    let arg = list.car();
    if let Number(NumberBox::Float(_)) = arg {
        Boolean(true)
    } else {
        Boolean(false)
    }
}

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

fn integer_eq(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 2) {
        panic!("integer=?: wrong integer of arguments");
    }
    let arg1 = list.car();
    let arg2 = list.cdr().car();
    if let Number(NumberBox::Integer(n1)) = arg1 {
        if let Number(NumberBox::Integer(n2)) = arg2 {
            Boolean(n1 == n2)
        } else {
            panic!("integer=?: not a integer");
        }
    } else {
        panic!("integer=?: not a integer");
    }
}

fn float_eq(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 2) {
        panic!("float=?: wrong float of arguments");
    }
    let arg1 = list.car();
    let arg2 = list.cdr().car();
    if let Number(NumberBox::Float(n1)) = arg1 {
        if let Number(NumberBox::Float(n2)) = arg2 {
            Boolean(n1 == n2)
        } else {
            panic!("float=?: not a float");
        }
    } else {
        panic!("float=?: not a float");
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
            Boolean(false)
        }
    } else {
        Boolean(false)
    }
}

fn float2integer(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 1) {
        panic!("float->integer: wrong float of arguments");
    }
    let arg = list.car();
    if let Number(NumberBox::Float(f)) = arg {
        LispType::integer_of(f as isize)
    } else {
        panic!("float->integer: not a float");
    }
}

fn float2string(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 1) {
        panic!("float->string: wrong float of arguments");
    }
    let arg = list.car();
    if let Number(NumberBox::Float(n)) = arg {
        Strings(n.to_string())
    } else {
        panic!("float->string: not a float");
    }
}

fn integer2float(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 1) {
        panic!("integer->float: wrong integer of arguments");
    }
    let arg = list.car();
    if let Number(NumberBox::Integer(n)) = arg {
        LispType::float_of(n as f64)
    } else {
        panic!("integer->float: not a integer");
    }
}

fn integer2string(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 1) {
        panic!("integer->string: wrong integer of arguments");
    }
    let arg = list.car();
    if let Number(NumberBox::Integer(n)) = arg {
        Strings(n.to_string())
    } else {
        panic!("integer->string: not a integer");
    }
}

fn integer2char(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 1) {
        panic!("integer->char: wrong integer of arguments");
    }
    let arg = list.car();
    if let Number(NumberBox::Integer(n)) = arg {
        Char(n as u8 as char)
    } else {
        panic!("integer->char: not a integer");
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

fn less_than(apply_args: &mut ApplyArgs) -> LispType {
    calc(apply_args, |a, b| if a < b { ONE } else { ZERO })
}

fn less_than_or_equal(apply_args: &mut ApplyArgs) -> LispType {
    calc(apply_args, |a, b| if a <= b { ONE } else { ZERO })
}

fn greater_than(apply_args: &mut ApplyArgs) -> LispType {
    calc(apply_args, |a, b| if a > b { ONE } else { ZERO })
}

fn greater_than_or_equal(apply_args: &mut ApplyArgs) -> LispType {
    calc(apply_args, |a, b| if a >= b { ONE } else { ZERO })
}

fn eq(apply_args: &mut ApplyArgs) -> LispType {
    calc(apply_args, |a, b| if a == b { ONE } else { ZERO })
}

fn rem(apply_args: &mut ApplyArgs) -> LispType {
    calc(apply_args, |a, b| a % b)
}

fn calc(apply_args: &mut ApplyArgs, f: fn(NumberBox, NumberBox) -> NumberBox) -> LispType {
    Number(
        apply_args
            .args()
            .data()
            .iter()
            .map(|x| -> NumberBox {
                match x {
                    LispType::Number(i) => i.clone(),
                    v => panic!("{} not a number", v),
                }
            })
            .reduce(f)
            .unwrap(),
    )
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("integer?", is_integer);
    env.reg_procedure("float?", is_float);
    env.reg_procedure("number?", is_number);
    env.reg_procedure("integer=?", integer_eq);
    env.reg_procedure("float=?", float_eq);
    env.reg_procedure("number=?", number_eq);
    env.reg_procedure("float->integer", float2integer);
    env.reg_procedure("float->string", float2string);
    env.reg_procedure("integer->float", integer2float);
    env.reg_procedure("integer->string", integer2string);
    env.reg_procedure("integer->char", integer2char);
    env.reg_procedure("+", add);
    env.reg_procedure("-", subtract);
    env.reg_procedure("*", multiply);
    env.reg_procedure("/", divide);
    env.reg_procedure("%", rem);
    env.reg_procedure("<", less_than);
    env.reg_procedure("<=", less_than_or_equal);
    env.reg_procedure(">", greater_than);
    env.reg_procedure(">=", greater_than_or_equal);
    env.reg_procedure("=", eq);
}
