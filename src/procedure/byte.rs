use super::*;

fn bytes2string(apply_args: &mut ApplyArgs) -> LispType {
    let a = apply_args.args().car();
    if let Vector(v,s) = a{
        let d :Vec<u8> = v.try_read().expect("locked err").iter().filter(|x| if let Byte(u) = x {true}else{false}).map(|x|if let Byte(o) = x {*o}else{0}).collect::<Vec<_>>();
        let  v = d.as_slice();
        let str = String::from_utf8_lossy(v).to_string();
        return Strings(str);
    }else{
        panic!("byte-vector->string: error")
    }
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("byte-vector->string", bytes2string);
}
