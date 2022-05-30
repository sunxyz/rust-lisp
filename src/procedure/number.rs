use super::*;

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
    env.reg_procedure("+", add);
    env.reg_procedure("-", subtract);
    env.reg_procedure("*", multiply);
    env.reg_procedure("/", divide);
}
