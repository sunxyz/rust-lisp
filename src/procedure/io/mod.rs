mod console;
mod file;
use super::*;


pub fn reg_procedure(env: &mut Env) {
   console::reg_procedure(env);
   file::reg_procedure(env);
}
