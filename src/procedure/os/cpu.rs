use super::*;

fn get_os_cpu_num(apply_args: &mut ApplyArgs) -> LispType {
    LispType::integer_of(num_cpus::get() as isize)
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("get-os-cpu-num", get_os_cpu_num);
}
