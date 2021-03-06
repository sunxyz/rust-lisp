use super::*;
use std::io::{ Write};
// char string ub bytevector

fn write_char(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if list.len() == 0 {
        panic!("write-char: wrong integer of arguments");
    } else {
        let w = list.car();
        if let Char(c) = w {
            if list.len() == 1 {
                std::io::stdout().write_all(c.to_string().as_bytes());
            } else if list.len() == 2 {
                let port = list.cdr().car();
                if let Output(io) = port {
                    io.lock().expect("locked err")
                        .write_all(c.to_string().as_bytes());
                } else {
                    panic!("write-char: not a port");
                }
            } else {
                panic!("write-char: wrong integer of arguments");
            }
        } else {
            panic!("write-char: not a char");
        }
    }
    Nil
}

fn write_string(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args().data();
    if list.len() == 0 {
        panic!("write-string: wrong integer of arguments");
    } else {
        let w = list.get(0).expect("write-string: wrong integer of arguments");
        if let Strings(s) = w {
            if list.len() == 1 {
                std::io::stdout().write_all(s.as_bytes());
            } else if list.len() < 5 {
                let mut start = 0;
                let mut end = s.len();
                let port = list.get(1).expect("write-string: wrong integer of arguments");
                if list.len() >= 3 {
                    start = get_usize(list.get(2).expect("write-string: wrong integer of arguments")).expect("write-string: wrong integer of arguments");
                   
                }
                if list.len() == 4 {
                    end = get_usize(list.get(3).expect("write-string: wrong integer of arguments")).expect("write-string: wrong integer of arguments");
                }
                let  str = &s[start..end];
              
                if let Output(io) = port {
                    io.lock().expect("locked err")
                        .write_all(str.as_bytes());
                } else {
                    panic!("write-string: not a port");
                }
            } else {
                panic!("write-string: wrong integer of arguments");
            }
        } else {
            panic!("write-string: not a string");
        }
    }
    Nil
}

fn write_byte_vector(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args().data();
    if list.len() == 0 {
        panic!("write-bytevector: wrong integer of arguments");
    } else {
        // check bytes
        let w = list.get(0).expect("write-bytevector: wrong integer of arguments");
        if let Vector(vec, _) = w {
            let bytes = vec
                .ref4read()
                .iter()
                .map(|x| {
                    get_int(x).expect("write-bytevector: not a integer") as u8
                })
                .collect::<Vec<u8>>();
            if list.len() == 1 {
                std::io::stdout().write_all(bytes.as_slice());
            } else if list.len() < 5 {
                let mut start = 0;
                let mut end = bytes.len();
                let port = list.get(1).expect("write-bytevector: wrong integer of arguments");
                if list.len() >= 3 {
                    let v = list.get(2).expect("write-bytevector: wrong integer of arguments");
                    start = get_usize(v).expect("write-bytevector: wrong integer of arguments");
                }
                if list.len() == 4 {
                    let v = list.get(3).expect("write-bytevector: wrong integer of arguments");
                    end = get_usize(v).expect("write-bytevector: wrong integer of arguments");
                }
                let  bytes = &bytes[start..end];
                if let Output(io) = port {
                    io.lock().expect("locked err")
                        .write_all(bytes);
                } else {
                    panic!("write-string: not a port");
                }
            } else {
                panic!("write-bytevector: wrong integer of arguments");
            }
        } else {
            panic!("write-bytevector: not a bytevector");
        }
    }
    Nil
}

fn write_u8(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if list.len() == 0 {
        panic!("write-u8: wrong integer of arguments");
    } else {
        let w = list.car();
        if let Byte(n) = w {
            if list.len() == 1 {
                std::io::stdout().write_all(&[n]);
            } else if list.len() == 2 {
                let port = list.cdr().car();
                if let Output(io) = port {
                    io.lock().expect("locked err").write_all(&[n]);
                } else {
                    panic!("write-u8: not a port");
                }
            } else {
                panic!("write-u8: wrong integer of arguments");
            }
        } else {
            panic!("write-u8: not a integer");
        }
    }
    Nil
}

fn newline(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if list.len() == 0 {
        println!();
    } else if list.len() == 1 {
        if let Output(w) = list.car() {
            w.lock().expect("locked err").write_all(b"\n").unwrap();
        } else {
            panic!("newline: argument must be output stream");
        }
    } else {
        panic!("newline: wrong integer of arguments");
    }
    Nil
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("write-char", write_char);
    env.reg_procedure("write-string", write_string);
    env.reg_procedure("write-byte-vector", write_byte_vector);
    env.reg_procedure("write-u8", write_u8);
    env.reg_procedure("newline", newline);
}
