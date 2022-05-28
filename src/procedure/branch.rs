use super::*;

fn if0(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.expr();
    if list.data().len() > 1 {
        let cond = list.car();
        let then = list.cdr().car();
        let else_ = if list.data().len() > 2 {
            Expr(list.cdr().cdr())
        } else {
            Nil
        };
        if let Boolean(b) = apply_args.inter(&cond) {
            if b {
                apply_args.inter(&then)
            } else {
                if let Expr(else__) = else_ {
                    apply_args.inter(&Expr(else__.clone()))
                } else {
                    Nil
                }
            }
        } else {
            panic!("if: cond is not boolean");
        }
    } else {
        panic!("if: wrong number of arguments");
    }
}


pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("if", if0);
}
