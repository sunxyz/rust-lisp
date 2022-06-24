use futures::StreamExt;

use super::*;

fn coroutine_start(apply_args: &mut ApplyArgs) -> LispType {
    apply_args.check_args_num(1);
    async_go(apply_args);
    Nil
}

async fn async_go(x: &mut ApplyArgs) -> LispType {
    x.inter(&LispType::Expr(x.expr().clone()))
}

fn make_channel(apply_args: &mut ApplyArgs) -> LispType {
    apply_args.check_args_num(1);
    if let Number(size) = apply_args.args().car() {
        LispType::make_channel(size as usize)
    } else {
        panic!("channel size must be number")
    }
}

fn coroutine_channel_read(apply_args: &mut ApplyArgs) -> LispType {
    apply_args.check_args_num(1);
    if let LispType::Channel(channel) = apply_args.args().car() {
        let v = channel.borrow_mut()
            .1
            .try_next()
            .expect("channel is empty")
            .expect("channel is empty");
        v
    } else {
        panic!("channel read must be channel")
    }
}


fn coroutine_channel_write(apply_args: &mut ApplyArgs) -> LispType {
    apply_args.check_args_num(2);
    let list = apply_args.args();
    let channel = list.car();
    let v_list  = list.cdr();
    if let LispType::Channel(channel) = channel {
        v_list.data().iter().for_each(|v| {
            channel.borrow_mut().0.try_send(v.clone()).expect("channel is full");
        });
        Nil
    } else {
        panic!("channel write must be channel")
    }
}

fn coroutine_channel_close(apply_args: &mut ApplyArgs) -> LispType {
    apply_args.check_args_num(1);
    if let LispType::Channel(channel) = apply_args.args().car() {
        channel.borrow_mut().1.close();
        Nil
    } else {
        panic!("channel close must be channel")
    }
}

fn sleep(apply_args: &mut ApplyArgs) -> LispType {
    apply_args.check_args_num(1);
    if let Number(n) = apply_args.args().car() {
        let n = n as u64;
        std::thread::sleep(std::time::Duration::from_millis(n));
        Nil
    } else {
        panic!("sleep must be number")
    }
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("run", coroutine_start);
    env.reg_procedure("make-chan", make_channel);
    env.reg_procedure("chan-read", coroutine_channel_read);
    env.reg_procedure("chan-write", coroutine_channel_write);
    env.reg_procedure("chan-close", coroutine_channel_close);
    env.reg_procedure("sleep", sleep);
}
