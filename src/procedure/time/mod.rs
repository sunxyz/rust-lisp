use super::*;
use std::time::{SystemTime, UNIX_EPOCH};
fn current_second(apply_args: &mut ApplyArgs) -> LispType {
    LispType::integer_of(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as isize,
    )
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("current-second", current_second);
}
