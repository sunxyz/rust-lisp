mod func;
mod list;

use std::cell::RefCell;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::rc::Rc;

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
    Cons(Rc<RefCell<Vec<LispType>>>),
    Vector(Rc<RefCell<Vec<LispType>>>, usize),
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
            LispType::Cons(c) => write!(
                f,
                "({} {})",
                c.borrow().get(0).unwrap(),
                c.borrow().get(1).unwrap()
            ),
            LispType::Vector(v, _) => write!(
                f,
                "#({})",
                v.borrow()
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
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
                LispType::Cons(m) => c == m,
                _ => false,
            },
            LispType::Vector(v, l) => match other {
                LispType::Vector(m, n) => v == m && l == n,
                _ => false,
            },
        }
    }
}
impl LispType {
    pub fn new_exp() -> LispType {
        LispType::Expr(List::new())
    }
    pub fn exp_of(l: List) -> LispType {
        LispType::Expr(l)
    }
}
// pub use self::atom::*;
