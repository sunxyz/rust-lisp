mod cons_box;
mod func;
mod list;

use std::cell::RefCell;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::fs::File;
use std::io::BufRead;
use std::io::Write;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

use futures::Future;
use futures::channel::mpsc::Receiver;
use futures::channel::mpsc::Sender;
use futures::channel::mpsc::channel;
use futures::lock::Mutex;

pub use self::func::ApplyArgs;
pub use self::cons_box::ConsBox;
pub use self::list::List;
pub type ProcedureBox = Rc<Box<dyn Fn(&mut ApplyArgs) -> LispType>>;
pub type VectorBox = Rc<RefCell<Vec<LispType>>>;
pub type InputBox =  Rc<RefCell<Box<dyn BufRead>>>;
pub type OutputBox = Rc<RefCell<Box<dyn Write>>>;
pub type ChannelBox= Arc<RefCell<(Sender<LispType>, Receiver<LispType>)>>;

pub enum LispType {
    Number(isize),
    Symbol(String),
    Strings(String),
    Char(char),
    Byte(u8),
    Boolean(bool),
    Nil,
    Cons(ConsBox),
    Expr(List),
    Procedure(ProcedureBox),
    Vector(VectorBox, usize),
    Input(InputBox),
    Output(OutputBox),
    Channel(ChannelBox),
}

impl Clone for LispType {
    fn clone(&self) -> Self {
        match self {
            LispType::Number(n) => LispType::Number(*n),
            LispType::Symbol(s) => LispType::Symbol(s.clone()),
            LispType::Strings(s) => LispType::Strings(s.clone()),
            LispType::Boolean(b) => LispType::Boolean(*b),
            LispType::Char(c) => LispType::Char(*c),
            LispType::Byte(u)=> LispType::Byte(*u),    
            LispType::Nil => LispType::Nil,
            LispType::Expr(l) => LispType::Expr(l.clone()),
            LispType::Procedure(f) => LispType::Procedure(f.clone()),
            LispType::Cons(c) => LispType::Cons(c.clone()),
            LispType::Vector(v, l) => LispType::Vector(v.clone(), l.clone()),
            LispType::Input(i) => LispType::Input(i.clone()),
            LispType::Output(o) => LispType::Output(o.clone()),
            LispType::Channel(c) => LispType::Channel(c.clone()),
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
            LispType::Byte(u) => write!(f, "{}", u),
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
            LispType::Input(_) => write!(f, "<port>"),
            LispType::Output(_) => write!(f, "<port>"),
            LispType::Channel(_) => write!(f, "<channel>"),
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
            LispType::Byte(u) => match other {
                LispType::Byte(m) => u == m,
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
            LispType::Channel(c) => match other {
                LispType::Channel(m) => c.as_ref() as *const _ == m.as_ref() as *const _,
                _ => false,
            },
        }
    }
}
impl LispType {
    pub fn cons_of(car: LispType, cdr: LispType) -> LispType {
        LispType::Cons(ConsBox::new(car, cdr))
    }
    pub fn expr_of(elem_s: Vec<LispType>) -> LispType {
        LispType::Expr(List::of(elem_s))
    }
    pub fn vector_of(vec: Vec<LispType>) -> LispType {
        let len = vec.len() as usize;
        LispType::Vector(Rc::new(RefCell::new(vec)), len)
    }
    pub fn procedure_of(f: Box<dyn Fn(&mut ApplyArgs) -> LispType>) -> LispType {
        LispType::Procedure(Rc::new(f))
    }
    pub fn input_of(input: Box<dyn BufRead>) -> LispType {
        LispType::Input(Rc::new(RefCell::new(input)))
    }
    pub fn output_of(output: Box<dyn Write>) -> LispType {
        LispType::Output(Rc::new(RefCell::new(output)))
    }
    pub fn make_channel(buffer: usize) -> LispType {
        LispType::Channel(Arc::new(RefCell::new(channel::<LispType>(buffer))))
    }
}
// pub use self::atom::*;
