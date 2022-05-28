mod define;
mod lambda;
mod number;
mod branch;
mod cons;
mod set;
mod vector;
mod list;
mod quote;
mod apply;

use crate::env::{Env, EnvOps};
use crate::t::LispType::*;
use crate::t::*;
use std::rc::Rc;

trait ProcedureRegister {
    fn reg_procedure(&mut self, name: &str, proc: fn(&mut ApplyArgs) -> LispType);
}

impl ProcedureRegister for Env {
    fn reg_procedure(&mut self, name: &str, proc: fn(&mut ApplyArgs) -> LispType) {
        self.define(
            name,
            Procedure(Rc::new(Box::new(proc))),
        );
    }
}

pub fn init_procedure(env: &mut Env) {
    number::reg_procedure(env);
    define::reg_procedure(env);
    set::reg_procedure(env);
    lambda::reg_procedure(env);
    branch::reg_procedure(env);
    cons::reg_procedure(env);
    vector::reg_procedure(env);
    list::reg_procedure(env);
    quote::reg_procedure(env);
    apply::reg_procedure(env);
}
