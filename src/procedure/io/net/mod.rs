use super::*;
mod tcp;

pub fn reg_procedure(env: &mut Env) {
    tcp::reg_procedure(env);
}
