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
mod types;
mod utils;

use env::{Env, EnvOps, RefEnv};
use interpreter::{eval, interpreter};
use parser::parser;
use std::{
    env as std_env,
    io::{self, Read, Write},
    ops::Add,
};
use types::{
    ApplyArgs,
    LispType::{self, Nil},
    RefOps,
};

use crate::types::List;

fn main() {
    args_handler();
}

fn args_handler() {
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
    procedure::init_procedure(&mut root.ref4write());
    let env = Env::extend(root);
    println!("\n\x1b[34m--------------------------\n welcome rust-lisp v0.1.0 \n--------------------------\x1b[0m \n \x1b[30msource code:https://github.com/sunxyz/rust-lisp \x1b[0m\n");
    let mut line = String::new();
    let mut expr_str = String::new();
    let mut exprs = String::new();

    loop {
        expr_str.clear();
        print!("\x1b[32m>\x1b[0m ");
        read_line(&mut line);
        while line.ends_with("\\") {
            expr_str.push_str(&line[..line.len() - 1]);
            read_line(&mut line);
        }
        if !line.ends_with("\\") {
            expr_str.push_str(&line);
        }
        if (expr_str.trim() == ":quit") {
            println!("\x1b[32m good bye! \x1b[0m ");
            break;
        } else if (expr_str.trim() == ":save") {
            save_file(&format!("({}\n)",exprs));
            continue;
        } else if (!expr_str.trim().starts_with("(")) {
            expr_str = format!("({})", expr_str);
        }
        exprs.push_str("\n\t");
        exprs.push_str(&expr_str);
        let exp = parser(expr_str.clone()).expect("parser error");
        let r = interpreter(exp, env.clone());
        if let Nil = r {
        } else {
            println!("{}", r);
        }
    }
}

fn read_line(line: &mut String) {
    line.clear();
    std::io::stdout().flush();
    std::io::stdin().read_line(line).unwrap();
    line.remove(line.len() - 1);
}

fn save_file(expr: &String) {
    println!("please input the file name:");
    let mut filename = String::new();
    read_line(&mut filename);
    let filename = filename.trim();
    let mut file = std::fs::File::create(filename).expect("create file failed");
    file.write_all(expr.to_string().as_bytes()).expect("save file failed");
    println!("\x1b[32m save file success\x1b[0m ");
}
