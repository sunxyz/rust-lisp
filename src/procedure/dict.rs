use std::collections::HashMap;

use super::*;

fn is_dict(apply_args: &mut ApplyArgs) -> LispType {
    let args = apply_args.args();
    if args.len()==1{
        if let Dict(dict) = args.car(){
            LispType::Boolean(true)
        }else{
            LispType::Boolean(false)
        }
    }else{
        panic!("is_dict: args length is not 1");
    }
}

fn dict_eq(apply_args: &mut ApplyArgs) -> LispType {
    let args = apply_args.args();
    if args.len()==2{
       LispType::Boolean(args.car().eq(&args.cdr().car()))
    }else{
        panic!("dict=?: args length is not 2");
    }
}


fn make_dict(apply_args: &mut ApplyArgs) -> LispType {
    LispType::dict_of(HashMap::new())
}

fn dict(apply_args: &mut ApplyArgs) -> LispType {
    let args = apply_args.args();
    if args.len()==2{
        let keys = args.car();
        let vals =args.cdr().car();
        let mut dict = HashMap::new();
        if let Expr(keys) = keys{
            if let Expr(vals) = vals{
                let len = keys.len();
                if len == vals.len(){
                    let keys = keys.data();
                    let vals = vals.data();
                    for i in 0..len{
                        dict.insert(keys[i].clone().to_string(), vals[i].clone());
                    };
                    LispType::dict_of(dict)
                }else{
                   panic!("dict: keys and vals length not equal")
                }
            }else{
                panic!("dict: vals is not list");
            }
        }else {
            panic!("dict: keys is not list");
        }
    }else{
        LispType::dict_of(HashMap::new())
    }
}




fn dict_rm(apply_args: &mut ApplyArgs) -> LispType {
    let args = apply_args.args();
    if args.len()==2{
        if let Dict( dict) = args.car(){
            let key = args.cdr().car();
            dict.ref4write().remove(&key.to_string());
            Nil
        }else{
            panic!("dict-rm!: car is not dict");
        }
    }else{
        panic!("dict-rm!: args length is not 2");
    }
}

fn dict_get(apply_args: &mut ApplyArgs) -> LispType {
    let args = apply_args.args();
    if args.len()==2{
        if let Dict( dict) = args.car(){
            let key = args.cdr().car();
            let key = if let Strings(key) = key{
                key
            }else{
                key.to_string()
            };
            if let Some(val) = dict.ref4read().get(&key){
                val.clone()
            }else{
                Nil
            }
        }else{
            panic!("dict-get: car is not dict");
        }
    }else{
        panic!("dict-get: args length is not 2");
    }
}

fn dict_put(apply_args: &mut ApplyArgs) -> LispType {
    let args = apply_args.args();
    if args.len()==3{
        if let Dict( dict) = args.car(){
            let key = args.cdr().car();
            let key = if let Strings(key) = key{
                key
            }else{
                key.to_string()
            };
            let val = args.cdr().cdr().car();
            dict.ref4write().insert(key, val.clone());
            Nil
        }else{
            panic!("dict-put!: car is not dict");
        }
    }else{
        panic!("dict-put!: args length is not 3");
    }
}

fn dict_clear(apply_args: &mut ApplyArgs) -> LispType {
    let args = apply_args.args();
    if args.len()==1{
        if let Dict( dict) = args.car(){
            dict.ref4write().clear();
            Nil
        }else{
            panic!("dict-clear!: car is not dict");
        }
    }else{
        panic!("dict-clear!: args length is not 1");
    }
}

fn dict_contains_key(apply_args: &mut ApplyArgs) -> LispType {
    let args = apply_args.args();
    if args.len()==2{
        if let Dict( dict) = args.car(){
            let key = args.cdr().car();
            let key = if let Strings(key) = key{
                key
            }else{
                key.to_string()
            };
            LispType::Boolean(dict.ref4read().contains_key(&key))
        }else{
            panic!("dict-contains-key: car is not dict");
        }
    }else{
        panic!("dict-contains-key: args length is not 2");
    }
}

fn dict_len(apply_args: &mut ApplyArgs) -> LispType {
    let args = apply_args.args();
    if args.len()==1{
        if let Dict( dict) = args.car(){
            LispType::Number(dict.ref4read().len() as isize)
        }else{
            panic!("dict-len: car is not dict");
        }
    }else{
        panic!("dict-len: args length is not 1");
    }
}

fn dict_keys2list(apply_args: &mut ApplyArgs) -> LispType {
    let args = apply_args.args();
    if args.len()==1{
        if let Dict( dict) = args.car(){
            let mut list = List::new();
            for key in dict.ref4read().keys(){
                list.push(Strings(key.to_string()));
            }
            Expr(list)
        }else{
            panic!("dict-keys->list: car is not dict");
        }
    }else{
        panic!("dict-keys->list: args length is not 1");
    }
}

fn dict_value2list(apply_args: &mut ApplyArgs) -> LispType {
    let args = apply_args.args();
    if args.len()==1{
        if let Dict( dict) = args.car(){
            let mut list = List::new();
            for val in dict.ref4read().values(){
                list.push(val.clone());
            }
            Expr(list)
        }else{
            panic!("dict-value->list: car is not dict");
        }
    }else{
        panic!("dict-value->list: args length is not 1");
    }
}




fn dict2list(apply_args: &mut ApplyArgs) -> LispType {
    let args = apply_args.args();
    if args.len()==1{
        if let Dict( dict) = args.car(){
            let mut list = List::new();
            for (key,val) in dict.ref4read().iter(){
                list.push(LispType::cons_of(Strings(key.to_string()), val.clone()));
            }
            Expr(list)
        }else{
            panic!("dict->list: car is not dict");
        }
    }else{
        panic!("dict->list: args length is not 1");
    }
}



pub fn reg_procedure(env: &mut Env) {

    env.reg_procedure("dict?", is_dict);
    env.reg_procedure("dict=?", dict_eq);
    env.reg_procedure("dict", dict);
    env.reg_procedure("make-dict", make_dict);
    env.reg_procedure("dict-length", dict_len);
    env.reg_procedure("dict-rm!", dict_rm);
    env.reg_procedure("dict-get", dict_get);
    env.reg_procedure("dict-put!", dict_put);
    env.reg_procedure("dict-clear！", dict_clear);
    env.reg_procedure("dict-contains-key?", dict_contains_key);
    env.reg_procedure("dict-keys->list", dict_keys2list);
    env.reg_procedure("dict-value->list", dict_value2list);
    env.reg_procedure("dict->list", dict2list);
}
