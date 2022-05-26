mod number;
mod define;

use crate::t::*;
use crate::t::LispType::*;
use crate::env::Env;

pub fn init_procedure(env :& mut Env) {
    number::reg_procedure(env);
    define::reg_procedure(env);
}