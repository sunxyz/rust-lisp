use super::*;
use crate::parser::parser;

fn eval(apply_args: &mut ApplyArgs) -> LispType {
    let mut result = Nil;
    apply_args.args().data().clone().iter().for_each(|arg| {
        if let Strings(contents) = arg {
            result = apply_args.inter(&Expr(parser(contents.to_string()).unwrap()));
        } else {
            panic!("eval: not string");
        }
    });
    result
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("eval", eval);
}
