
use crate::types::{LispType::{self, Number}, NumberBox::{self, *}};


pub fn get_int(v: &LispType) -> Result<isize, String> {
    match v {
        Number(n) => {
            match n {
                NumberBox::Integer(i) => {
                    return Ok(*i);
                }
                NumberBox::Float(f) => {
                    return Err(format!("{} is not an integer", f));
                }
            }
        }
        _ => {
            return Err(format!("{} is not an integer", v));
        }
    }
}

pub fn get_float(v: &LispType) -> Result<f64, String> {
    match v {
        Number(n) => {
            match n {
                NumberBox::Integer(i) => {
                    return Ok(*i as f64);
                }
                NumberBox::Float(f) => {
                    return Ok(*f);
                }
            }
        }
        _ => {
            return Err(format!("{} is not a float", v));
        }
    }
}

pub fn get_usize(v: &LispType) -> Result<usize, String> {
    match v {
        Number(n) => {
            match n {
                NumberBox::Integer(i) => {
                    return Ok(i.abs() as usize);
                }
                NumberBox::Float(f) => {
                    return Err(format!("{} is not an integer", f));
                }
            }
        }
        _ => {
            return Err(format!("{} is not an integer", v));
        }
    }
}