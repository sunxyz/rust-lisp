use std::{
    io::{BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

use super::*;

// call-with-tcp-listener "127.0.0.1:8089" proc
fn call_with_tcp_listener(apply_args: &mut ApplyArgs) -> LispType {
    apply_args.check_args_num(2);
    let args = apply_args.args();
    if let Strings(addr) = args.car() {
        if let Procedure(proc) = args.cdr().car() {
            // let p = proc.as_ref();
            let listener = TcpListener::bind(addr).unwrap();
            for stream in listener.incoming() {
                let stream = stream.unwrap();

                handle_connection(stream, apply_args, &proc);
            }
        } else {
            panic!("call_with_tcp_listener: not a proc")
        }
    } else {
        panic!("call_with_tcp_listener: not a addr")
    }
    Nil
}

fn handle_connection(
    mut stream: TcpStream,
    apply_args: &mut ApplyArgs,
    proc: &ProcedureBox,
) {
    // let mut buffer = [0; 1024];
    // stream.read(&mut buffer).unwrap();
    let read = BufReader::new(stream.try_clone().expect("tcp stream error"));
    let input = LispType::input_of(Box::new(read));
    // stream
    let args = Some(List::of(vec![input]));
    let res = proc.borrow()(&mut apply_args.clone_of(args));

    let response = res.to_string();
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("call-with-tcp-listener", call_with_tcp_listener);
}
