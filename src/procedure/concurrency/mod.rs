mod thread;
mod lock;
mod channel;
mod barrier;


use super::LispType;
use super::*;


pub fn reg_procedure(env: &mut Env) {
    thread::reg_procedure(env);
    lock::reg_procedure(env);
    barrier::reg_procedure(env);
    channel::reg_procedure(env);
}
