use std::{
    io::{self, BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
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
            // listener
            //     .set_nonblocking(true)
            //     .expect("Cannot set non-blocking");
            for stream in listener.incoming() {
                match stream {
                    Ok(s) => {
                        let apply_args = apply_args.clone();
                        let proc = proc.clone();
                        handle_connection(s, apply_args, proc);
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        // wait until network socket is ready, typically implemented
                        // via platform-specific APIs such as epoll or IOCP
                        continue;
                    }
                    Err(e) => panic!("encountered IO error: {}", e),
                }
            }
        } else {
            panic!("call_with_tcp_listener: not a proc")
        }
    } else {
        panic!("call_with_tcp_listener: not a addr")
    }
    Nil
}

fn handle_connection(mut stream: TcpStream, mut apply_args: ApplyArgs, proc: ProcedureBox) {
    let mut reader = BufReader::new(stream.try_clone().expect("tcp stream error"));
    let writer = Box::new(stream.try_clone().expect("tcp stream error"));

    let mut buffer = [0; 0];
    reader.read(&mut buffer).unwrap();

    let res = proc.ref4read()(&mut apply_args.clone_of(Some(List::of(vec![
        LispType::input_of(Box::new(reader)),
        LispType::output_of(writer),
    ]))));
    if let Nil = res {}
    if let Concurrency(_) = res {
    } else {
        let response = if let Strings(str) = res {
            str
        } else {
            res.to_string()
        };
        if stream.write(response.as_bytes()).is_ok() {
            stream.flush().expect("failed to flush stream");
        }else {
            // 重试
            if(stream.write(response.as_bytes()).is_ok()){
                stream.flush().expect("failed to flush stream");
            }
        }
    }
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("call-with-tcp-listener", call_with_tcp_listener);
}
