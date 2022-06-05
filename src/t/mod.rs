mod cons_;
mod func;
mod list;

use std::cell::RefCell;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::rc::Rc;

pub use self::cons_::Cons_;
pub use self::func::ApplyArgs;
pub use self::list::List;

pub enum LispType {
    Number(i32),
    Symbol(String),
    Strings(String),
    Boolean(bool),
    Char(char),
    Nil,
    Expr(List),
    Procedure(Rc<Box<dyn Fn(&mut ApplyArgs) -> LispType>>),
    Cons(Cons_),
    Vector(Rc<RefCell<Vec<LispType>>>, usize),
    Input(Rc<RefCell<Box<dyn Read>>>),
    Output(Rc<RefCell<Box<dyn Write>>>),
}

impl Clone for LispType {
    fn clone(&self) -> Self {
        match self {
            LispType::Number(n) => LispType::Number(*n),
            LispType::Symbol(s) => LispType::Symbol(s.clone()),
            LispType::Strings(s) => LispType::Strings(s.clone()),
            LispType::Boolean(b) => LispType::Boolean(*b),
            LispType::Char(c) => LispType::Char(*c),
            LispType::Nil => LispType::Nil,
            LispType::Expr(l) => LispType::Expr(l.clone()),
            LispType::Procedure(f) => LispType::Procedure(f.clone()),
            LispType::Cons(c) => LispType::Cons(c.clone()),
            LispType::Vector(v, l) => LispType::Vector(v.clone(), l.clone()),
            LispType::Input(i) => LispType::Input(i.clone()),
            LispType::Output(o) => LispType::Output(o.clone()),
        }
    }
}

impl Display for LispType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            LispType::Number(i) => write!(f, "{}", i),
            LispType::Symbol(s) => write!(f, "{}", s),
            LispType::Strings(s) => write!(f, "{}", s),
            LispType::Boolean(b) => write!(f, "{}", if *b { "#t" } else { "#f" }),
            LispType::Char(c) => write!(f, "{}", c),
            LispType::Nil => write!(f, "nil"),
            LispType::Expr(l) => write!(f, "{}", l),
            LispType::Procedure(_) => write!(f, "<procedure>"),
            LispType::Cons(c) => write!(f, "{}", &c.to_string()),
            LispType::Vector(v, _) => write!(
                f,
                "#({})",
                v.borrow()
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            LispType::Input(_) => write!(f, "<input>"),
            LispType::Output(_) => write!(f, "<output>"),
        }
    }
}

impl PartialEq for LispType {
    fn eq(&self, other: &LispType) -> bool {
        match self {
            LispType::Number(n) => match other {
                LispType::Number(m) => n == m,
                _ => false,
            },
            LispType::Symbol(s) => match other {
                LispType::Symbol(m) => s == m,
                _ => false,
            },
            LispType::Strings(s) => match other {
                LispType::Strings(m) => s == m,
                _ => false,
            },
            LispType::Boolean(b) => match other {
                LispType::Boolean(m) => b == m,
                _ => false,
            },
            LispType::Char(c) => match other {
                LispType::Char(m) => c == m,
                _ => false,
            },
            LispType::Nil => match other {
                LispType::Nil => true,
                _ => false,
            },
            LispType::Expr(l) => match other {
                LispType::Expr(m) => l == m,
                _ => false,
            },
            LispType::Procedure(f) => match other {
                LispType::Procedure(m) => f.as_ref() as *const _ == m.as_ref() as *const _,
                _ => false,
            },
            LispType::Cons(c) => match other {
                LispType::Cons(m) => c.eq(m),
                _ => false,
            },
            LispType::Vector(v, l) => match other {
                LispType::Vector(m, n) => v == m && l == n,
                _ => false,
            },
            LispType::Input(i) => match other {
                LispType::Input(m) => i.as_ref() as *const _ == m.as_ref() as *const _,
                _ => false,
            },
            LispType::Output(o) => match other {
                LispType::Output(m) => o.as_ref() as *const _ == m.as_ref() as *const _,
                _ => false,
            },
        }
    }
}
impl LispType {
    pub fn cons_of(car: LispType, cdr: LispType) -> LispType {
        Cons_::new(car, cdr)
    }
    pub fn vector_of(vec: Vec<LispType>) -> LispType {
        let len = vec.len() as usize;
        LispType::Vector(Rc::new(RefCell::new(vec)), len)
    }
    pub fn input_of(input: Box<dyn Read>) -> LispType {
        LispType::Input(Rc::new(RefCell::new(input)))
    }
    pub fn output_of(output: Box<dyn Write>) -> LispType {
        LispType::Output(Rc::new(RefCell::new(output)))
    }

}
// pub use self::atom::*;
