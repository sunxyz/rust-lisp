use super::*;

fn make_channel(apply_args: &mut ApplyArgs) -> LispType {
    LispType::make_concurrency_channel()
}

fn channel_send(apply_args: &mut ApplyArgs) -> LispType {
    apply_args.check_args_num(2);
    let list = apply_args.args();
    let car = list.car();
    let cdr = list.cdr();
    if let Concurrency(ConcurrencyBox::Channel(tx, rx)) = car {
        tx.send(cdr.car()).expect("channel send error");
        Nil
    } else {
        panic!("channel-send: not a channel");
    }
}

fn channel_done(apply_args: &mut ApplyArgs) -> LispType {
    apply_args.check_args_num(1);
    let list = apply_args.args();
    let car = list.car();
    if let Concurrency(ConcurrencyBox::Channel(tx, rx)) = car {
        tx.send(Symbol("done".to_string())).expect("channel send error");
        Nil
    } else {
        panic!("channel-send: not a channel");
    }
}

fn channel_recv(apply_args: &mut ApplyArgs) -> LispType {
    apply_args.check_args_num(1);
    let car = apply_args.args().car();
    if let Concurrency(ConcurrencyBox::Channel(tx, rx)) = car {
        rx.recv().expect("channel recv error")
    } else {
        panic!("channel-recv: not a channel");
    }
}


fn channel_foreach(apply_args: &mut ApplyArgs) -> LispType {
    apply_args.check_args_num(2);
    let list = apply_args.args();
    let proc =list.car();
    let chan = list.cdr().car();
    if let Concurrency(ConcurrencyBox::Channel(tx, rx)) = chan {
        if let Procedure(proc)=proc{
            for x in rx.iter() {
                if(x==Symbol("done".to_string())){
                    return Nil;
                }
                let mut apply_args0 = apply_args.clone_of(Some(List::of(vec![x])));
                proc.read().expect("proc error")(&mut apply_args0);
            }
            Nil
        }else {
            panic!("channel-for-each: not a procedure");
        }
    } else {
        panic!("channel-for-each: not a channel");
    }
}

fn channel_map(apply_args: &mut ApplyArgs) -> LispType {
    apply_args.check_args_num(2);
    let list = apply_args.args();
    let proc =list.car();
    let chan = list.cdr().car();
    let mut result = List::new();
    if let Concurrency(ConcurrencyBox::Channel(tx, rx)) = chan {
        if let Procedure(proc)=proc{
            for x in rx.iter() {
                if(x==Symbol("done".to_string())){
                    return Expr(result);
                }
                let mut apply_args0 = apply_args.clone_of(Some(List::of(vec![x])));
                result.push(proc.read().expect("proc error")(&mut apply_args0));
            }
            Expr(result)
        }else {
            panic!("channel-for-each: not a procedure");
        }
    } else {
        panic!("channel-for-each: not a channel");
    }
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("make-channel", make_channel);
    env.reg_procedure("channel-send", channel_send);
    env.reg_procedure("->", channel_send);
    env.reg_procedure("channel-done", channel_done);
    env.reg_procedure("channel-recv", channel_recv);
    env.reg_procedure("<-", channel_recv);
    env.reg_procedure("channel-for-each", channel_foreach);
    env.reg_procedure("<-for-each", channel_foreach);
    env.reg_procedure("channel-map", channel_map);
    env.reg_procedure("<--", channel_map);
}
