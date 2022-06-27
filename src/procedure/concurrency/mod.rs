use std::{
    sync::{Barrier, Mutex},
    thread,
};
use super::LispType;
use super::*;

fn thread_run(apply_args: &mut ApplyArgs) -> LispType {
    let proc = apply_args.args().car();
    let args = apply_args.args().cdr();
    if let Procedure(proc) = proc {
        let mut apply_args0 = apply_args.clone_of(Some(args));
        let join_handler = thread::Builder::new()
            .name("thread".into())
            .spawn(move || {
                proc.read().expect("proc error")(&mut apply_args0)
            });
        LispType::concurrency_thread_of(join_handler.expect("thread error"))
    } else {
        panic!("thread-run: not a procedure");
    }
}



fn thread_join(apply_args: &mut ApplyArgs) -> LispType {
    let thread = apply_args.args().pop();
    if let Concurrency(ConcurrencyBox::THREAD(t)) = thread {
        // t.join().expect("thread join error")
        Nil
    } else {
        panic!("thread-join: not a thread");
    }
}

fn make_lock(apply_args: &mut ApplyArgs) -> LispType {
    apply_args.check_args_num(1);
    let car = apply_args.args().car();
    LispType::concurrency_lock_of(car)
}

fn make_barrier(apply_args: &mut ApplyArgs) -> LispType {
    apply_args.check_args_num(1);
    let car = apply_args.args().car();
    if let Number(size) = car {
        LispType::concurrency_barrier_of(size as usize)
    } else {
        panic!("make-barrier: not a number");
    }
}

fn barrier_await(apply_args: &mut ApplyArgs) -> LispType {
    apply_args.check_args_num(1);
    let car = apply_args.args().car();
    if let Concurrency(ConcurrencyBox::BARRIER(barrier)) = car {
        barrier.try_read().expect("barrier error").wait();
        LispType::Nil
    } else {
        panic!("barrier-await: not a barrier");
    }
}

fn sleep(apply_args: &mut ApplyArgs) -> LispType {
    apply_args.check_args_num(1);
    let num = apply_args.args().car();
    if let Number(num) = num {
        thread::sleep(std::time::Duration::from_millis(num as u64));
        Nil
    } else {
        panic!("sleep: not a number");
    }
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("thread-run", thread_run);
    env.reg_procedure("sleep", sleep);
    // env.reg_procedure("thread-join", thread_join);
    env.reg_procedure("make-lock", make_lock);
    env.reg_procedure("make-barrier", make_barrier);
    env.reg_procedure("barrier-await", barrier_await);
}
