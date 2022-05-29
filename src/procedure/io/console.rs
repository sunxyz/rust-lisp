use super::*;

fn display(apply_args: &mut ApplyArgs) -> LispType {
    let args = apply_args.args();
    let mut s = "".to_string();
    for arg in args.data().iter() {
        s.push_str(&format!("{}", arg));
    }
    print!("{}", s);
    Nil
}

fn newline(apply_args: &mut ApplyArgs) -> LispType {
    println!();
    Nil
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("display", display);
    env.reg_procedure("newline", newline);
}
