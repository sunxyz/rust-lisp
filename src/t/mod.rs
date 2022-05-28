mod func;
mod list;

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::rc::Rc;
use std::cell::RefCell;

pub use self::func::ApplyArgs;
pub use self::list::List;

pub enum LispType {
    Number(i32),
    Symbol(String),
    String(String),
    Boolean(bool),
    Char(char),
    Nil,
    Expr(List),
    Procedure(Rc<Box<dyn Fn(&mut ApplyArgs) -> LispType>>),
    Cons(Rc<RefCell<Vec<LispType>>>),
    Vector(Rc<RefCell<Vec<LispType>>>,usize),
}

impl Clone for LispType {
    fn clone(&self) -> Self {
        match self {
            LispType::Number(n) => LispType::Number(*n),
            LispType::Symbol(s) => LispType::Symbol(s.clone()),
            LispType::String(s) => LispType::String(s.clone()),
            LispType::Boolean(b) => LispType::Boolean(*b),
            LispType::Char(c) => LispType::Char(*c),
            LispType::Nil => LispType::Nil,
            LispType::Expr(l) => LispType::Expr(l.clone()),
            LispType::Procedure(f) => LispType::Procedure(f.clone()),
            LispType::Cons(c) => LispType::Cons(c.clone()),
            LispType::Vector(v,l) => LispType::Vector(v.clone(),l.clone()),
        }
    }
}

impl Display for LispType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            LispType::Number(i) => write!(f, "{}", i),
            LispType::Symbol(s) => write!(f, "`{}", s),
            LispType::String(s) => write!(f, "{}", s),
            LispType::Boolean(b) => write!(f, "{}", if *b { "#t" } else { "#f" }),
            LispType::Char(c) => write!(f, "{}", c),
            LispType::Nil => write!(f, "nil"),
            LispType::Expr(l) => write!(f, "{}", l),
            LispType::Procedure(_) => write!(f, "<procedure>"),
            LispType::Cons(c) => write!(f, "({} {})", c.borrow().get(0).unwrap(), c.borrow().get(1).unwrap()),
            LispType::Vector(v,_) => write!(f, "#({})", v.borrow().iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")),
        }
    }
}
// pub use self::atom::*;
