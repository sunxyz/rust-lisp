use super::*;
fn define(apply_args: &mut ApplyArgs) -> LispType {
    let expr = apply_args.expr();
    let car = expr.car();
    let var_name = if let Symbol(_) = car {car} else{ apply_args.clone().inter(& car)};
    if let Symbol(key) = var_name{
        let v = apply_args.clone().inter(&Expr(expr.cdr()));
        apply_args.env().borrow_mut().define(key.as_str(), v);
        Nil
    } else {
        panic!("define: invalid argument");
    }
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("define", define);
}
