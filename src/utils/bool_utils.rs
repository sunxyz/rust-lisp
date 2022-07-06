use crate::types::{LispType::{self, *}, NumberBox};

pub fn is_true(v: &LispType) -> bool {
    let v = v.clone();
    match v {
        Boolean(b) => {
            return b;
        }
        Number(n) => {
           match n {
               NumberBox::Integer(i) => {
                   return i != 0;
               }
               NumberBox::Float(f) => {
                   return f != 0.0;
               }
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
