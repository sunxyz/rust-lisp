use super::*;
fn define(apply_args: &mut ApplyArgs) -> LispType {
    let expr = apply_args.expr();
    if let Symbol(key) = expr.car() {
        let v = apply_args.inter(&Expr(expr.cdr()));
        apply_args.env().borrow_mut().define(key.as_str(), v);
        Nil
    } else {
        panic!("define: invalid argument");
    }
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("define", define);
}
