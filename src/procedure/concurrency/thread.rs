use super::*;
use std::{
    thread,
};

fn thread_run(apply_args: &mut ApplyArgs) -> LispType {
    let proc = apply_args.args().car();
    let args = apply_args.args().cdr();
    if let Procedure(proc) = proc {
        let mut apply_args0 = apply_args.clone_of(Some(args));
        let join_handler = thread::Builder::new()
            .name("thread".into())
            .spawn(move || proc.read().expect("proc error")(&mut apply_args0));
        LispType::concurrency_thread_of(join_handler.expect("thread error"))
    } else {
        panic!("thread-run: not a procedure");
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
}
