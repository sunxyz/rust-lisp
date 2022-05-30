use crate::t::LispType::{self, *};

pub fn is_true(v: &LispType) -> bool {
    let v = v.clone();
    match v {
        Boolean(b) => {
            return b;
        }
        Number(n) => {
            if n != 0 {
                return true;
            }
        }
        Strings(s) => {
            if s != "" || s != "nil" || s != "0" || s == "#t" {
                return true;
            }
        }
        Char(c) => {
            if c != '\0' {
                return true;
            }
        }
        Nil => {}
        _ => {
            return true;
        }
    }
    return false;
}
