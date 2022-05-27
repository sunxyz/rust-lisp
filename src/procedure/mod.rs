mod number;
mod define;
mod lambda;

use std::rc::Rc;
use crate::t::*;
use crate::t::LispType::*;
use crate::env::Env;
use crate::env::EnvOption;


trait ProcedureRegister {
    fn reg_procedure(&mut self, name: &str, proc: fn(&mut  ApplyArgs) -> LispType);
}

impl ProcedureRegister for Env {
    fn reg_procedure(&mut self, name: &str, proc: fn(&mut  ApplyArgs) -> LispType) {
        // let t = Func::new(name.to_string(), None, None, Some(proc));
        self.set(name, Procedure(Rc::new(Box::new(proc))));
    }
}

pub fn init_procedure(env :& mut Env) {
    number::reg_procedure(env);
    define::reg_procedure(env);
    lambda::reg_procedure(env);
}