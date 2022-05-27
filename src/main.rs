#![allow(unused_imports)]
#![allow(unreachable_patterns)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
#![allow(dead_code)]
#![allow(type_alias_bounds)]
#![allow(unconditional_recursion)]
#![allow(mutable_borrow_reservation_conflict)]
// #![allow(non_snake_case)]

mod env;
mod interpreter;
mod parse;
mod procedure;
mod t;
use interpreter::eval;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
fn main() {
    // let s = Rc::new(RefCell::new("我很善变，还拥有多个主人".to_string()));

    // let s1 = s.clone();
    // let s2 = s.clone();
    // let s3 = Some(s2);
    // // let mut s2 = s.borrow_mut();
    // let t = s3.unwrap();
    // // s2.borrow_mut().push_str("，我很善变，还拥有多个主人");
    // t.borrow_mut().push_str(", on yeah!");
    // t.borrow_mut().push_str(", on yeah!");
    // s1.borrow_mut().push_str(", on yeah!");
    // s1.borrow_mut().push_str(", on yeah!");
    // s1.borrow_mut().push_str(", on yeah!");
    // println!("{:?}\n{:?}\n{:?}", s, s1, t);

    // println!("Hello, world!");
    // t::t();
    // print!("{}",parse::parse("((+ 1 2 3 5 6 7  (15) (- 0 1 2 3 5 6 7 (10))))").unwrap());
    // let r = parse::parse("(+ 1 2 3  7 8 9  5 6 7 (10))").unwrap();
    // println!("{}", r);
    // println!("{}", r.cdr());

    let r = eval("((define a 10)(+ a 1 2 3 5 6 7  (15) (- 0 1 2 3 5 6 7 (10))))");
    println!("{}", r.ok().unwrap());

    // let mut j = [1] ;
    // let k = j;
    // j[0] = 2;
    // println!("{ } ",  r);
    // for i in k {
    //     println!("{}", i);
    // }
    // interpreter::print_type_of(r);
    // interpreter::interpreter(r);
    // - 1 2 3
    // - - 4 5 6
    // - 7 8 9
    // - - 10
    // let mut ts = String::from("sss");
    // ts = t(ts);
    // println!("{}", ts);
    // let tr = &mut ts[..];
    // rs(tr);
    // println!("{}", tr);
}

//
use t::LispType;

pub trait EnvOption {
    fn get(&self, key: &str) -> Option<LispType>;
    fn set(&mut self, key: &str, value: LispType);
    fn fork(&self) -> Rc<RefCell<Env>>;
}

pub enum Env {
    Empty,
    Extend(Rc<RefCell<EnvContainer>>),
}

impl EnvOption for Env {
    fn fork(&self) -> Rc<RefCell<Env>> {
        match self {
            Env::Extend(frame) => frame.borrow_mut().fork(),
            _ => panic!("fork on empty env"),
        }
    }
    fn get(&self, key: &str) -> Option<LispType> {
        match self {
            Env::Empty => None,
            Env::Extend(frame) => frame.borrow_mut().get(key),
        }
    }
    fn set(&mut self, key: &str, value: LispType) {
        match self {
            Env::Empty => panic!("set on empty env"),
            Env::Extend(frame) => frame.borrow_mut().set(key, value),
        }
    }
}

pub struct EnvContainer {
    parent: Rc<RefCell<Env>>,
    env: HashMap<String, LispType>,
    self_: Rc<RefCell<Env>>,
}

impl EnvContainer {
    pub fn new(parent: Rc<RefCell<Env>>) -> Rc<RefCell<Env>> {
        let f = EnvContainer {
            parent: parent,
            env: HashMap::new(),
            self_: Rc::new(RefCell::new(Env::Empty)),
        };
        let self_ = Rc::new(RefCell::new(f));
        let env = Env::Extend(self_.clone());
        let self_ = Rc::new(RefCell::new(env));
        self_.clone()
    }
}

impl EnvOption for EnvContainer {
    fn fork(&self) -> Rc<RefCell<Env>> {
        Self::new(self.self_.clone())
    }
    fn get(&self, key: &str) -> Option<LispType> {
        if self.env.contains_key(key) {
            Some(self.env[key].clone())
        } else {
            self.parent.borrow_mut().get(key)
        }
    }
    fn set(&mut self, key: &str, value: LispType) {
        self.env.insert(key.to_string(), value);
    }
}
