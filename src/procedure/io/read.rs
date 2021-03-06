use std::io::Read;
use std::io::{BufRead, BufReader};

use super::*;

fn read_char(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    let mut v = [0u8; 1];
    if list.len() == 0 {
        std::io::stdin().read_exact(&mut v);
        LispType::Char(v[0] as char)
    } else if list.len() == 1 {
        // let mut f = BufReader::new(File::open("input.txt").expect("open failed"));
        let port = list.car();
        if let Input(io) = port {
            let t = io.to_owned();
            io.lock().expect("locked err").read_exact(&mut v);
        } else {
            panic!("read-char: not a port");
        }
        LispType::Char(v[0] as char)
    } else {
        panic!("read-char: wrong integer of arguments")
    }
}

fn read_line(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    let mut v = String::new();
    if list.len() == 0 {
        std::io::stdin().read_line(&mut v);
        LispType::Strings(v)
    } else if list.len() == 1 {
        let port = list.car();
        if let Input(io) = port {
            // todo: read_line
            io.lock().expect("locked err").read_line(&mut v);
        } else {
            panic!("read-line: not a port");
        }
        LispType::Strings(v)
    } else {
        panic!("read-line: wrong integer of arguments")
    }
}

fn read_string(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    let mut v = String::new();
    if list.len() == 0 {
        std::io::stdin().read_to_string(&mut v);
        LispType::Strings(v)
    } else if list.len() == 1 {
        let port = list.car();
        if let Input(io) = port {
            io.lock().expect("locked err").read_to_string(&mut v);
        } else {
            panic!("read-string: not a port");
        }
        LispType::Strings(v)
    } else {
        panic!("read-string: wrong integer of arguments")
    }
}

fn read_u8(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if list.len() == 0 {
        let mut v = [0u8; 1];
        std::io::stdin().read_exact(&mut v);
        LispType::integer_of(v[0] as isize)
    } else if list.len() == 1 {
        let port = list.car();
        if let Input(io) = port {
            let mut v = [0u8; 1];
            io.lock().expect("locked err").read_exact(&mut v);
            LispType::Byte(v[0] as u8)
        } else {
            panic!("read-u8: not a port");
        }
    } else {
        panic!("read-u8: wrong integer of arguments")
    }
}

fn read_byte_vector(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if list.len() < 2 {
        let k = if list.len() == 1 {
            get_usize(&list.car()).expect("read-byte-vector: not an integer")
        } else {
            1024
        };
        let mut v = vec![0u8; k];
        std::io::stdin().read_exact(&mut v);
        let vec = v.iter().map(|x| Byte(x.clone())).collect::<Vec<LispType>>();
        return LispType::vector_of(vec);
    } else if list.len() == 2 {
        let port = list.car();
        if let Input(io) = port {
            let k =  get_usize(&list.cdr().car() ).expect("read-byte-vector: not a port");
            let mut v = vec![0u8; k];
                let buf = v.as_mut_slice();
                // io.try_lock().expect("locked err").expect("io error").read_exact(&mut v);
                io.lock().expect("locked err").read(buf);
                let vec = buf
                    .iter()
                    .map(|x| Byte(x.clone()))
                    .collect::<Vec<LispType>>();
                return LispType::vector_of(vec);
        } else {
            panic!("read-byte-vector: wrong integer of arguments")
        }
    } else {
        panic!("read-byte-vector: wrong integer of arguments")
    }
}

fn read_byte_vector_end(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() == 0) {
        let mut vec = Vec::new();
        std::io::stdin().read_to_end(&mut vec);
        let vec = vec.into_iter().map(|x| Byte(x)).collect::<Vec<LispType>>();
        return LispType::vector_of(vec);
    } else if list.len() == 1 {
        let port = list.car();
        if let Input(io) = port {
            let mut vec = Vec::new();
            io.lock().expect("locked err").read_to_end(&mut vec);
            let vec = vec.into_iter().map(|x| Byte(x)).collect::<Vec<LispType>>();
            return LispType::vector_of(vec);
        } else {
            panic!("read-byte-vector-end: not a port")
        }
    } else {
        panic!("read-byte-vector-end: not a port")
    }
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("read-char", read_char);
    env.reg_procedure("read-line", read_line);
    env.reg_procedure("read-string", read_string);
    env.reg_procedure("read-u8", read_u8);
    env.reg_procedure("read-byte-vector", read_byte_vector);
    env.reg_procedure("read-byte-vector-end", read_byte_vector_end);
}
