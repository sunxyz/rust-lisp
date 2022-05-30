use super::*;
use crate::parser::parser;
use crate::utils::file_utils::read_file;

fn load(apply_args: &mut ApplyArgs) -> LispType {
    let mut result = Nil;
    apply_args.args().clone().for_each(|arg| {
        let mut contents = "".to_string();
        if let Strings(filename) = arg {
            read_file("./Cargo.toml").split("\n").for_each(|line| {
                let line = line.trim();
                if line.starts_with("//") {
                    return;
                }
                if (!(line.starts_with(";") || line.starts_with("#"))) {
                    contents.push_str(line);
                }
            });
        }else {
            panic!("load: not file");
        }
        result = apply_args.inter(&Expr(parser(contents).unwrap()));
    });
    result
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("load", load);
}
