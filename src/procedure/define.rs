use super::*;
fn define(apply_args: &mut ApplyArgs) -> LispType{
    let expr = apply_args.expr();
    if let Symbol(key) = expr.car() {
        let v = apply_args.eval(&expr.cdr().car());
        apply_args.env().set(key.as_str(), v);
        Nil
    } else {
        panic!("define: invalid argument");
    }
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("define", define);
}