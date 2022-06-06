use super::*;

fn is_input_port(apply_args: &mut ApplyArgs) -> LispType {
    apply_args.check_args_num(1);
    let port = apply_args.args().car();
    if let Input(port) = port {
        Boolean(true)
    } else {
        Boolean(false)
    }
}

fn is_output_port(apply_args: &mut ApplyArgs) -> LispType {
    apply_args.check_args_num(1);
    let port = apply_args.args().car();
    if let Output(port) = port {
        Boolean(true)
    } else {
        Boolean(false)
    }
}

fn is_port(apply_args: &mut ApplyArgs) -> LispType {
    apply_args.check_args_num(1);
    let port = apply_args.args().car();
    if let Input(port) = port {
        Boolean(true)
    } else if let Output(port) = port {
        Boolean(true)
    } else {
        Boolean(false)
    }
}


pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("input-port?", is_input_port);
    env.reg_procedure("output-port?", is_output_port);
    env.reg_procedure("port?", is_port);
}
