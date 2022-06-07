use super::*;
use crate::parser::parser;

fn eval(apply_args: &mut ApplyArgs) -> LispType {
    let mut result = Nil;
    apply_args.args().data().iter().for_each(|arg| {
        if let Strings(contents) = arg {
            let mut exp_str = String::new();
            contents.split("\n").for_each(|line| {
                let line = line.trim();
                if line.starts_with("//") {
                    return;
                }
                if (!(line.starts_with(";") || line.starts_with("#"))) {
                    exp_str.push_str(format!(" {}",line).as_str());
                }
            });
            result = apply_args.inter(&Expr(parser(exp_str).unwrap()));
        } else {
            panic!("eval: not string");
        }
    });
    result
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("eval", eval);
}
