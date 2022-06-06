use std::{fs::File, io::{BufReader, Write}};

use super::*;

fn call_with_input_file(apply_args: &mut ApplyArgs) -> LispType {
    apply_args.check_args_num(2);
    let filename = apply_args.args().car();
    let proc = apply_args.args().cdr().car();
    if let Strings(filename) = filename {
        if let Procedure(proc) = proc {
            let f = File::open(filename).expect("file not found");
            let read = BufReader::new(f);
            let read = LispType::input_of(Box::new(read));
            let mut args = List::new();
            args.push(read);
            let mut a = apply_args.clone_of(Some(args));
            proc(&mut a);
        } else {
            panic!("Invalid procedure");
        }
    } else {
        panic!("Invalid filename: {}", filename);
    }
    Nil
}

fn call_with_output_file(apply_args: &mut ApplyArgs) -> LispType {
    apply_args.check_args_num(2);
    let filename = apply_args.args().car();
    let proc = apply_args.args().cdr().car();
    if let Strings(filename) = filename {
        if let Procedure(proc) = proc {
            let f = File::create(filename).expect("file not found");
            let write = LispType::output_of(Box::new(f));
            let mut args = List::new();
            args.push(write);
            let mut a = apply_args.clone_of(Some(args));
            proc(&mut a);
        } else {
            panic!("Invalid procedure");
        }
    } else {
        panic!("Invalid filename: {}", filename);
    }
    Nil
}

fn open_input_file(apply_args: &mut ApplyArgs) -> LispType {
    apply_args.check_args_num(1);
    let filename = apply_args.args().car();
    if let Strings(filename) = filename {
        let f = File::open(filename).expect("file not found");
        let read = BufReader::new(f);
        LispType::input_of(Box::new(read))
    } else {
        panic!("Invalid filename: {}", filename);
    }
}

fn open_output_file(apply_args: &mut ApplyArgs) -> LispType {
    apply_args.check_args_num(1);
    let filename = apply_args.args().car();
    if let Strings(filename) = filename {
        let f = File::create(filename).expect("file not found");
        LispType::output_of(Box::new(f))
    } else {
        panic!("Invalid filename: {}", filename);
    }
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("call-with-input-file", call_with_input_file);
    env.reg_procedure("call-with-output-file", call_with_output_file);
    env.reg_procedure("open-input-file", open_input_file);
    env.reg_procedure("open-output-file", open_output_file);
}
