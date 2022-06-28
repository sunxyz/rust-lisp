use super::*;

fn make_barrier(apply_args: &mut ApplyArgs) -> LispType {
    apply_args.check_args_num(1);
    let car = apply_args.args().car();
    if let Number(size) = car {
        LispType::concurrency_barrier_of(size as usize)
    } else {
        panic!("make-barrier: not a number");
    }
}

fn barrier_wait(apply_args: &mut ApplyArgs) -> LispType {
    apply_args.check_args_num(1);
    let car = apply_args.args().car();
    if let Concurrency(ConcurrencyBox::BARRIER(barrier)) = car {
        barrier.try_read().expect("barrier error").wait();
        LispType::Nil
    } else {
        panic!("barrier-await: not a barrier");
    }
}
pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("make-barrier", make_barrier);
    env.reg_procedure("barrier-wait", barrier_wait);
}
