mod number;
mod define;

use crate::t::*;
use crate::t::LispType::*;
use crate::env::Env;
use crate::env::EnvOption;

trait ProcedureRegister {
    fn reg_procedure(&mut self, name: &str, proc: fn(&mut  ApplyArgs) -> LispType);
}

impl ProcedureRegister for Env {
    fn reg_procedure(&mut self, name: &str, proc: fn(&mut  ApplyArgs) -> LispType) {
        self.set(name, Procedure(proc));
    }
}

pub fn init_procedure(env :& mut Env) {
    number::reg_procedure(env);
    define::reg_procedure(env);
}