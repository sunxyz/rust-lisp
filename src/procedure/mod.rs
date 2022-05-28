mod define;
mod lambda;
mod number;

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
    lambda::reg_procedure(env);
}
