use super::*;
fn define(apply_args: &mut ApplyArgs) -> LispType {
    let expr = apply_args.expr();
    if let Symbol(key) = expr.car() {
        let v = apply_args.inter(&Expr(expr.cdr()));
        if apply_args.env().get(key.as_str()).is_none() {
            apply_args.env().define(key.as_str(), v);
        } else {
            panic!("define: {} duplicate definition", key);
        }
        Nil
    } else {
        panic!("define: invalid argument");
    }
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("define", define);
}
