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

use interpreter::eval;
use std::env as std_env;
use t::LispType;
fn main() {
    let args: Vec<String> = std_env::args().collect();
    // println!("{:?}", args);
    if (args.len() < 3) {
        println!("Usage: lisp [file]");
        return;
    }
    let run_model = &args[1];
    let filename = &args[2];
    if (run_model == "run") {
        eval(format!("(load '{}')", filename).as_str());
    } else {
        println!("model check :{}", "run or proc");
    }
}
