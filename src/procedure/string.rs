use super::*;
use crate::utils::bool_utils::is_true;
use std::cell::RefCell;

fn is_string(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 1) {
        panic!("is_string: wrong number of arguments");
    }
    let arg = list.car();
    if let Strings(s) = arg {
        Boolean(true)
    } else {
        Boolean(false)
    }
}

fn mark_string(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() < 1) {
        panic!("mark_string: wrong number of arguments");
    }
    let size = get_usize(&list.car()).expect("mark-string: invalid argument");
    if (list.len() == 2) {
        let arg = list.cdr().car();
        if let Char(c) = arg {
            Strings(c.to_string().repeat(size))
        } else {
            panic!("mark-string: not a string");
        }
    } else {
        Strings(" ".repeat(size))
    }
}

fn string_eq(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 2) {
        panic!("string=?: wrong number of arguments");
    }
    let arg1 = list.car();
    let arg2 = list.cdr().car();
    if let Strings(s1) = arg1 {
        if let Strings(s2) = arg2 {
            Boolean(s1 == s2)
        } else {
            panic!("string=?: not a string");
        }
    } else {
        panic!("string=?: not a string");
    }
}

fn string_length(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 1) {
        panic!("string_length: wrong number of arguments");
    }
    let arg = list.car();
    if let Strings(s) = arg {
        LispType::integer_of(s.len() as isize)
    } else {
        panic!("string_length: not a string");
    }
}

fn string_ref(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 2) {
        panic!("string-ref: wrong number of arguments");
    }
    let arg = list.car();
    if let Strings(s) = arg {
        let n = get_int(&list.cdr().car()).expect("string-ref: invalid argument");
        if (n < 0) || (n >= s.len() as isize) {
            panic!("string-ref: index out of range");
        }
        Char(s.chars().nth(n as usize).unwrap())
    } else {
        panic!("string-ref: not a string");
    }
}

fn string_set(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 3) {
        panic!("string_set: wrong number of arguments");
    }
    let arg = list.car();
    if let Strings(s) = arg {
        let index = list.cdr().car();
        let n = get_int(&index).expect("string-set: invalid argument");
        if (n < 0) || (n >= s.len() as isize) {
            panic!("string_set: index out of range");
        }
        let c = list.cdr().cdr().car();
        if let Char(c) = c {
            let v = Strings(
                s.chars()
                    .take(n as usize)
                    .chain(c.to_string().chars())
                    .chain(s.chars().skip(n as usize + 1))
                    .collect(),
            );
            if let Symbol(var) = apply_args.expr().car() {
                apply_args.env().ref4write().set(&var, v);
                Nil
            } else {
                panic!("string-set!: not a symbol");
            }
        } else {
            panic!("string-set!: not a char");
        }
    } else {
        panic!("string-set!: not a string");
    }
}

fn substring(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 3) {
        panic!("substring: wrong number of arguments");
    }
    let arg = list.car();
    if let Strings(s) = arg {
        let start = get_int(&list.cdr().car()).expect("substring: invalid argument");
        if (start < 0) || (start >= s.len() as isize) {
            panic!("substring: start out of range");
        }
        let end = get_int(&list.cdr().cdr().car()).expect("substring: not a end");
        if (end < 0) || (end >= s.len() as isize) {
            panic!("substring: end out of range");
        }
        if (end < start) {
            panic!("substring: end < start");
        }
        Strings(s[start as usize..end as usize].to_string())
    } else {
        panic!("substring: not a string");
    }
}

fn string_append(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() < 2) {
        panic!("string-append: wrong number of arguments");
    }
    let s = list.car();
    let last = list.cdr();
    if let Strings(s) = s {
        let mut result = s.clone();
        for arg in last.data() {
            if let Strings(s) = arg {
                result.push_str(&s);
            } else {
                result.push_str(&arg.to_string());
            }
        }
        Strings(result)
    } else {
        panic!("string-append: not a string");
    }
}

fn string_copy(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if list.len() < 1 {
        panic!("string-copy: wrong number of arguments");
    }
    let arg = list.car();
    if let Strings(s) = arg {
        let mut start0 = 0;
        let mut end0 = s.len();
        if list.len() > 1 {
            let start = list.cdr().car();
            let start = get_int(&start).expect("string-copy: not a start");
            if (start < 0) || (start >= s.len() as isize) {
                panic!("string-copy: start out of range");
            }
            start0 = start;
        }
        if list.len() > 2 {
            let end = list.cdr().cdr().car();
            let end = get_int(&end).expect("string-copy: not a end");
            if (end < 0) || (end >= s.len() as isize) {
                panic!("string-copy: end out of range");
            }
            if (end < start0) {
                panic!("string-copy: end < start");
            }
            end0 = end as usize;
        }
        Strings(s[start0 as usize..end0].to_string())
    } else {
        panic!("string-copy: not a string");
    }
}

fn string_find(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 3) {
        panic!("string-find: wrong number of arguments");
    }
    let arg = list.car();
    if let Strings(s) = arg {
        let str = list.cdr().car();
        if let Strings(c) = str {
            s.find(c.as_str())
                .map(|n| LispType::integer_of(n as isize))
                .unwrap_or(LispType::integer_of(-1))
        } else {
            panic!("string-find: not a string");
        }
    } else {
        panic!("string-find: not a string");
    }
}

fn string_trim(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 1) {
        panic!("string-trim: wrong number of arguments");
    }
    let arg = list.car();
    if let Strings(s) = arg {
        Strings(s.trim().to_string())
    } else {
        panic!("string-trim: not a string");
    }
}

fn string_replace(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 3) {
        panic!("string-replace: wrong number of arguments");
    }
    let arg = list.car();
    if let Strings(s) = arg {
        let str = list.cdr().car();
        if let Strings(c) = str {
            let new = list.cdr().cdr().car();
            if let Strings(n) = new {
                Strings(s.replace(c.as_str(), n.as_str()))
            } else {
                panic!("string-replace: not to string");
            }
        } else {
            panic!("string-replace: not from string");
        }
    } else {
        panic!("string-replace: not source string");
    }
}

fn string2list(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 1) {
        panic!("string->list: wrong number of arguments");
    }
    let arg = list.car();
    if let Strings(s) = arg {
        let mut result = List::new();
        for c in s.chars().rev() {
            result.push(Char(c));
        }
        Expr(result)
    } else {
        panic!("string->list: not a string");
    }
}

fn string2symbol(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 1) {
        panic!("string->symbol: wrong number of arguments");
    }
    let arg = list.car();
    if let Strings(s) = arg {
        Symbol(s.clone())
    } else {
        panic!("string->symbol: not a string");
    }
}

fn string2vector(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 1) {
        panic!("string->vector: wrong number of arguments");
    }
    let arg = list.car();
    if let Strings(s) = arg {
        let mut result = Vec::new();
        for c in s.chars() {
            result.push(Char(c));
        }
        LispType::vector_of(result)
    } else {
        panic!("string->vector: not a string");
    }
}

fn string2number(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 1) {
        panic!("string->number: wrong number of arguments");
    }
    let arg = list.car();
    if let Strings(s) = arg {
        let n = s.parse::<isize>().unwrap();
        LispType::integer_of(n)
    } else {
        panic!("string->number: not a string");
    }
}

fn string2bool(apply_args: &mut ApplyArgs) -> LispType {
    let list = apply_args.args();
    if (list.len() != 1) {
        panic!("string->bool: wrong number of arguments");
    }
    let arg = list.car();
    if let Strings(_) = arg.clone() {
        Boolean(is_true(&arg))
    } else {
        panic!("string->boolean: not a string");
    }
}

pub fn reg_procedure(env: &mut Env) {
    env.reg_procedure("string?", is_string);
    env.reg_procedure("string=?", string_eq);
    env.reg_procedure("mark-string", mark_string);
    env.reg_procedure("string-length", string_length);
    env.reg_procedure("string-ref", string_ref);
    env.reg_procedure("string-set!", string_set);
    env.reg_procedure("substring", substring);
    env.reg_procedure("string-append", string_append);
    env.reg_procedure("string-copy", string_copy);
    env.reg_procedure("string-find", string_find);
    env.reg_procedure("string-trim", string_trim);
    env.reg_procedure("string-replace", string_replace);
    env.reg_procedure("string->list", string2list);
    env.reg_procedure("string->symbol", string2symbol);
    env.reg_procedure("string->vector", string2vector);
    env.reg_procedure("string->number", string2number);
    env.reg_procedure("string->bool", string2bool);
}
