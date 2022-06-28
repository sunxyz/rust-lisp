use super::*;


fn make_lock(apply_args: &mut ApplyArgs) -> LispType {
    apply_args.check_args_num(1);
    let car = apply_args.args().car();
    LispType::concurrency_lock_of(car)
}

fn lock_exp(apply_args: &mut ApplyArgs) -> LispType {
    apply_args.check_args_num(2);
    let lock = apply_args.expr().car();
    let cdr = apply_args.expr().cdr();
    if let Concurrency(ConcurrencyBox::LOCK(l)) = lock {
        l.lock().expect("lock error");
        apply_args.inter(&Expr(cdr))
    } else {
        panic!("lock-acquire: not a lock");
    }
}


pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("make-lock", make_lock);
    env.reg_procedure("lock-exp", lock_exp);
}
