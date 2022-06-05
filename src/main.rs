#![allow(unused_imports)]
#![allow(unreachable_patterns)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
#![allow(dead_code)]
#![allow(type_alias_bounds)]
#![allow(unconditional_recursion)]
#![allow(mutable_borrow_reservation_conflict)]

// #![allow(non_snake_case)]

mod env;
mod interpreter;
mod parser;
mod procedure;
mod t;
mod utils;

use env::{Env, EnvOps, RefEnv};
use interpreter::{eval, interpreter};
use parser::parser;
use std::{
    env as std_env,
    io::{self, Read, Write},
};
use t::LispType::{self,Nil};

fn main() {
    args_handler();
}

fn args_handler(){
    let args: Vec<String> = std_env::args().collect();
    // println!("{:?}", args);

    if (args.len() < 2) {
        cmd_handler();
    } else {
        let run_model = &args[1];
        if (run_model == "run") {
            if (args.len() < 3) {
                println!("Usage: lisp [file]");
                return;
            }
            let filename = &args[2];
            eval(format!("(load '{}')", filename).as_str());
        } else if run_model == "cmd" {
            cmd_handler();
        } else {
            println!("Usage: lisp [run|cmd]");
        }
    }
}

fn cmd_handler() {

    let root = Env::root();
    procedure::init_procedure(&mut root.borrow_mut());
    let env = Env::extend(root);
    println!("\n\x1b[34m--------------------------\n welcome rust-lisp v0.1.0 \n--------------------------\x1b[0m \n \x1b[30msource code:https://github.com/sunxyz/rust-lisp \x1b[0m\n");
    let mut line = String::new();
    loop {
        line.clear();
        print!("> ");
        std::io::stdout().flush();
        std::io::stdin().read_line(&mut line).unwrap();
        if(line.trim() == "exit") {
            break;
        }else if(!line.trim().starts_with("(")) {
            line = format!("({})", line);
        }
        let exp = parser(line.clone()).expect("parser error");
        let r = interpreter(exp, env.clone());
        if let Nil = r {
        } else {
            println!("{}", r);
        }
    }
}
