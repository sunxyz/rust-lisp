mod define;
mod let_;
mod set;

mod nil;
mod boolean;
mod number;
mod char;
mod string;
mod symbol;
mod byte;
mod cons;
mod vector;
mod list;
mod dict;
mod quote;

mod lambda;
mod apply;
mod do_;
mod branch;
mod procedure;
mod type_;

mod define_macro;
mod load;
mod eval;

mod lazy_evaluation;
mod time;
mod io;
mod concurrency;
mod os;


use crate::env::{Env, EnvOps, RefEnv};
use crate::types::{LispType::{self,*}, List, ApplyArgs, ref_::{self,*}, ProcedureBox, ConcurrencyBox};

trait ProcedureRegister {
    fn reg_procedure(&mut self, name: &str, proc: fn(&mut ApplyArgs) -> LispType);
}

impl ProcedureRegister for Env {
    fn reg_procedure(&mut self, name: &str, proc: fn(&mut ApplyArgs) -> LispType) {
        self.define(name, LispType::procedure_of(Box::new(proc)));
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
   define::reg_procedure(env);
    let_::reg_procedure(env);
    set::reg_procedure(env);

    nil::reg_procedure(env);
    boolean::reg_procedure(env);
    number::reg_procedure(env);
    char::reg_procedure(env);
    string::reg_procedure(env);
    symbol::reg_procedure(env);
    byte::reg_procedure(env);
    cons::reg_procedure(env);
    vector::reg_procedure(env);
    list::reg_procedure(env);
    dict::reg_procedure(env);
    quote::reg_procedure(env);
    
    lambda::reg_procedure(env);
    apply::reg_procedure(env);
    
    do_::reg_procedure(env);
    branch::reg_procedure(env);
    
    procedure::reg_procedure(env);
    type_::reg_procedure(env);
    
    define_macro::reg_procedure(env);
    
    load::reg_procedure(env);
    eval::reg_procedure(env);
    
    lazy_evaluation::reg_procedure(env);
    time::reg_procedure(env);
    io::reg_procedure(env);
    concurrency::reg_procedure(env);
    os::reg_procedure(env);
}
