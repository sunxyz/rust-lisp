mod cons_box;
mod number_box;
mod func;
mod list;
mod concurrency;
pub mod ref_;

use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::io::BufRead;
use std::io::Write;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Barrier;
use std::sync::Mutex;
use std::sync::RwLock;
use std::thread;
pub use self::ref_::IRef;
pub use self::ref_::RefOps;


pub use self::func::ApplyArgs;
pub use self::cons_box::ConsBox;
pub use self::list::List;
pub use self::concurrency::ConcurrencyBox;
pub use self::number_box::NumberBox;

pub type ProcedureBox =IRef<Box<dyn Fn(&mut ApplyArgs) -> LispType + Sync + Send>>;
pub type VectorBox = IRef<Vec<LispType>>;
pub type InputBox =  Arc<Mutex<Box<dyn BufRead + Send>>>;
pub type OutputBox = Arc<Mutex<Box<dyn Write + Send>>>;
pub type DictBox = IRef<HashMap<String, LispType>>;

pub enum LispType {
    Number(NumberBox),
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
    Dict(DictBox),
    Input(InputBox),
    Output(OutputBox),
    Concurrency(ConcurrencyBox),
}

impl Clone for LispType {
    fn clone(&self) -> Self {
        match self {
            LispType::Number(n) => LispType::Number(n.clone()),
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
            LispType::Concurrency(c) => LispType::Concurrency(c.clone()),
            LispType::Dict(d) => LispType::Dict(d.clone()),
        }
    }
}

impl Display for LispType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            LispType::Number(n) => write!(f, "{}", n),
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
                v.ref4read()
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            LispType::Dict(i) => write!(f, "({})", i.ref4read().iter().map(|(k, v)| format!("({} {})", k, v)).collect::<Vec<String>>().join(" ")),
            LispType::Input(_) => write!(f, "<port>"),
            LispType::Output(_) => write!(f, "<port>"),
            LispType::Concurrency(c) => write!(f, "{}", c),
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
                LispType::Vector(m, n) => v.as_ref() as *const _ == m.as_ref() as *const _&& l == n,
                _ => false,
            },
            LispType::Dict(d) => match other {
                LispType::Dict(m) => d.as_ref() as *const _ == m.as_ref() as *const _,
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
            LispType::Concurrency(c) => match other {
                LispType::Concurrency(m) => m.eq(c),
                _ => false,
            },
        }
    }
}
impl LispType {
    pub fn integer_of(n:isize)->LispType{
        LispType::Number(NumberBox::Integer(n))
    }
    pub fn float_of(n:f64)->LispType{
        LispType::Number(NumberBox::Float(n))
    }
    pub fn cons_of(car: LispType, cdr: LispType) -> LispType {
        LispType::Cons(ConsBox::new(car, cdr))
    }
    pub fn expr_of(elem_s: Vec<LispType>) -> LispType {
        LispType::Expr(List::of(elem_s))
    }
    pub fn vector_of(vec: Vec<LispType>) -> LispType {
        let len = vec.len() as usize;
        LispType::Vector(ref_::new(vec), len)
    }
    pub fn procedure_of(f: Box<dyn Fn(&mut ApplyArgs) -> LispType + Sync + Send>) -> LispType {
        LispType::Procedure(ref_::new(f))
    }
    pub fn dict_of(dict: HashMap<String, LispType>) -> LispType {
        LispType::Dict(ref_::new(dict))
    }
    pub fn input_of(input: Box<dyn BufRead + Send>) -> LispType {
        LispType::Input(Arc::new(Mutex::new(input)))
    }
    pub fn output_of(output: Box<dyn Write + Send>) -> LispType {
        LispType::Output(Arc::new(Mutex::new(output)))
    }
    pub fn concurrency_thread_of(join_handler: thread::JoinHandle<LispType>) -> LispType {
        LispType::Concurrency(ConcurrencyBox::THREAD( Arc::new(Mutex::new(Some(join_handler)))))
    }
    pub fn concurrency_lock_of(lock: LispType) -> LispType {
        LispType::Concurrency(ConcurrencyBox::LOCK(Arc::new(Mutex::new(lock))))
    }
    pub fn concurrency_barrier_of(size: usize) -> LispType {
        LispType::Concurrency(ConcurrencyBox::BARRIER(Arc::new(RwLock::new(Barrier::new(size)))))
    }
    pub fn make_concurrency_channel() -> LispType {
        let (tx, tr) = flume::unbounded();
        LispType::Concurrency(ConcurrencyBox::Channel(tx, tr))
    }
}
// pub use self::atom::*;
