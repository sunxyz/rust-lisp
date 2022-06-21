mod console;
mod port;
mod file;
mod read;
mod write;
mod net;
use super::*;


pub fn reg_procedure(env: &mut Env) {
   console::reg_procedure(env);
   port::reg_procedure(env);
   file::reg_procedure(env);
   read::reg_procedure(env);
   write::reg_procedure(env);
   net::reg_procedure(env);
}
