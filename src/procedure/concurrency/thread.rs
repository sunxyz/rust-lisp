
use super::*;
use std::{
    thread, sync::{atomic::AtomicUsize, Mutex},
};

pub static THREAD_COUNT: AtomicUsize = AtomicUsize::new(1);

fn thread_run(apply_args: &mut ApplyArgs) -> LispType {
    let proc = apply_args.args().car();
    let args = apply_args.args().cdr();
    if let Procedure(proc) = proc {
        let mut apply_args0 = apply_args.clone_of(Some(args));

        let join_handler = thread::Builder::new()
            .name(format!("thread{}",THREAD_COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst)).to_string())
            .spawn(move || proc.ref4read()(&mut apply_args0));
        LispType::concurrency_thread_of(join_handler.expect("thread error"))
    } else {
        panic!("thread-run: not a procedure");
    }
}

fn sleep(apply_args: &mut ApplyArgs) -> LispType {
    apply_args.check_args_num(1);
    let num = apply_args.args().car();
    let num = get_usize(&num).expect("sleep: invalid argument");
    thread::sleep(std::time::Duration::from_millis(num as u64));
    Nil
}

fn join(apply_args: &mut ApplyArgs) -> LispType {
    apply_args.check_args_num(1);
    let car = apply_args.args().car();
    if let Concurrency(ConcurrencyBox::THREAD(car)) = car {
        let t = car.lock().expect("locked error").take().expect("thread error");
        t.join().expect("thread join error")
    } else {
        panic!("join: not a thread");
    }
}

fn integer(apply_args: &mut ApplyArgs) -> LispType{
   Strings( thread::current().name().unwrap_or("<unnamed>").to_string())
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("thread-run", thread_run);
    env.reg_procedure("current-thread-name", integer);
    env.reg_procedure("sleep", sleep);
    env.reg_procedure("join", join);
}
