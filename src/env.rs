use super::t::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::fmt::{Display, Formatter};
use std::fmt::Result;

pub trait EnvOption {
    fn get(&self, key: &str, is_find_up: bool) -> Option<LispType>;
    fn set(&mut self, key: &str, value: LispType);
    fn fork(&self) -> Rc<RefCell<Env>>;
}

pub enum Env {
    Empty,
    Extend(Rc<RefCell<EnvContainer>>),
}

impl Env {
    pub fn new() -> Rc<RefCell<Env>> {
        EnvContainer::new(Rc::new(RefCell::new(Env::Empty)))
    }
    fn set_self(&mut self, _self: Rc<RefCell<Env>>) {
        match self {
            Env::Extend(frame) => frame.borrow_mut().set_self(_self),
            _ => panic!("set_self on empty env"),
        }
    }
}

impl EnvOption for Env {
    fn fork(&self) -> Rc<RefCell<Env>> {
        match self {
            Env::Extend(frame) => frame.borrow_mut().fork(),
            _ => panic!("fork on empty env"),
        }
    }
    fn get(&self, key: &str, is_find_up: bool) -> Option<LispType> {
        match self {
            Env::Extend(frame) => frame.borrow_mut().get(key, is_find_up),
            _ => None,
        }
    }
    fn set(&mut self, key: &str, value: LispType) {
        match self {
            Env::Extend(frame) => frame.borrow_mut().set(key, value),
            _ => panic!("set on empty env"),
        }
    }
}

pub struct EnvContainer {
    parent: Rc<RefCell<Env>>,
    env: HashMap<String, LispType>,
    _self: Rc<RefCell<Env>>,
}

impl EnvContainer {
    pub fn new(parent: Rc<RefCell<Env>>) -> Rc<RefCell<Env>> {
        let f = EnvContainer {
            parent: parent,
            env: HashMap::new(),
            _self: Rc::new(RefCell::new(Env::Empty)),
        };
        let _self = Rc::new(RefCell::new(f));
        let env = Env::Extend(_self.clone());
        let r = _self.clone();
        let _self = Rc::new(RefCell::new(env));
        _self.borrow_mut().set_self(_self.clone());
        _self
    }
    fn set_self(&mut self, _self: Rc<RefCell<Env>>) {
        self._self = _self;
    }
}

impl EnvOption for EnvContainer {
    fn fork(&self) -> Rc<RefCell<Env>> {
        Self::new(self._self.clone())
    }
    fn get(&self, key: &str, is_find_up: bool) -> Option<LispType> {
        if (is_find_up) {
            if self.env.contains_key(key) {
                Some(self.env[key].clone())
            } else {
                self.parent.borrow_mut().get(key, is_find_up)
            }
        } else {
            if self.env.contains_key(key) {
                Some(self.env[key].clone())
            } else {
                None
            }
        }
    }
    fn set(&mut self, key: &str, value: LispType) {
        self.env.insert(key.to_string(), value);
    }
}


impl Display for Env {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Env::Empty => write!(f, "Empty"),
            Env::Extend(frame) => write!(f, "Extend({})", frame.borrow()),
        }
    }
}

impl Display for EnvContainer {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "EnvContainer ")
    }
}