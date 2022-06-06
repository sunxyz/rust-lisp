mod define;
mod let_;
mod lambda;
mod number;
mod branch;
mod cons;
mod set;
mod vector;
mod list;
mod quote;
mod apply;
mod define_macro;
mod io;
mod load;
mod eval;
mod boolean;
mod string;
mod symbol;
mod char;
mod procedure;
mod type_;
mod nil;
mod lazy_evaluation;
mod do_;

use crate::env::{Env, EnvOps, RefEnv};
use crate::t::LispType::*;
use crate::t::*;
use std::rc::Rc;

trait ProcedureRegister {
    fn reg_procedure(&mut self, name: &str, proc: fn(&mut ApplyArgs) -> LispType);
}

impl ProcedureRegister for Env {
    fn reg_procedure(&mut self, name: &str, proc: fn(&mut ApplyArgs) -> LispType) {
        self.define(name, Procedure(Rc::new(Box::new(proc))));
    }
}

impl ApplyArgs {
    fn check_args_num(&mut self, num: usize) {
        if self.args().len() != num {
            panic!("args num error");
        }
    }
}

pub fn init_procedure(env: &mut Env) {
    number::reg_procedure(env);
    boolean::reg_procedure(env);
    char::reg_procedure(env);
    symbol::reg_procedure(env);
    string::reg_procedure(env);
    cons::reg_procedure(env);
    list::reg_procedure(env);
    vector::reg_procedure(env);
    quote::reg_procedure(env);
    procedure::reg_procedure(env);
    nil::reg_procedure(env);
    type_::reg_procedure(env);

    define::reg_procedure(env);
    let_::reg_procedure(env);
    set::reg_procedure(env);
    lambda::reg_procedure(env);
    branch::reg_procedure(env);
    apply::reg_procedure(env);
    define_macro::reg_procedure(env);
    io::reg_procedure(env);
    load::reg_procedure(env);
    eval::reg_procedure(env);
    lazy_evaluation::reg_procedure(env);
    do_::reg_procedure(env);
}
