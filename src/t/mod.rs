mod list;
mod func;

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::rc::Rc;

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
    Procedure(Rc<Box<dyn Fn(&mut ApplyArgs)-> LispType>>),
    // Cons(Cons)
}


impl Clone for LispType{
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
            // LispType::Cons(c) => LispType::Cons(c.clone()),
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
            // LispType::Cons(c) => write!(f, "{}", c),
        }
    }
}
// pub use self::atom::*;

