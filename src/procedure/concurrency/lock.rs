use std::sync::Mutex;

use super::*;

fn make_lock(apply_args: &mut ApplyArgs) -> LispType {
    apply_args.check_args_num(1);
    let car = apply_args.args().car();
    LispType::concurrency_lock_of(car)
}

fn lock_exp(apply_args: &mut ApplyArgs) -> LispType {
    apply_args.check_args_num(2);
    let exp = apply_args.expr().car();
    let lock = apply_args.expr().cdr().car();
    if let Concurrency(ConcurrencyBox::LOCK(l)) = lock {
        l.lock().expect("lock error");
        apply_args.inter(&exp)
    } else {
        panic!("lock-exp: not a lock");
    }
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("make-lock", make_lock);
    env.reg_procedure("lock-exp", lock_exp);
}
