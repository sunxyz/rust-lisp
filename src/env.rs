use super::t::*;
use std::{cell::RefCell, cell::RefMut, collections::HashMap, rc::Rc};

pub enum EnvType {
    E(Env),
    None,
}

impl EnvType {
    pub fn new_rc() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(EnvType::E(Env {
            env: HashMap::new(),
            parent: Rc::new(RefCell::new(EnvType::None)),
        })))
    }
    pub fn extend(&mut self) -> Rc<RefCell<EnvType>> {
        match self {
            EnvType::E(env) => env.extend(),
            EnvType::None => {
                panic!("extend on None")
            }
        }
    }
    pub fn set(&mut self, key: &str, value: LispType) {
        match self {
            EnvType::E(env) => {
                env.set(key, value);
            }
            EnvType::None => {}
        }
    }
    pub fn get(&self, key: &str) -> Option<LispType> {
        match self {
            EnvType::E(env) => env.get(key),
            EnvType::None => None,
        }
    }

    pub fn set_parent(&mut self, parent: Rc<RefCell<EnvType>>) {
        match self {
            EnvType::E(env) => {
                env.set_parent(parent);
            }
            EnvType::None => {}
        }
    }
    pub fn env(&self, key: &str) -> Option<LispType> {
        match self {
            EnvType::E(env) => env.get(key),
            EnvType::None => None,
        }
    }
}


pub struct Env {
    env: HashMap<String, LispType>,
    parent: Rc<RefCell<EnvType>>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            env: HashMap::new(),
            parent: Rc::new(RefCell::new(EnvType::None)),
        }
    }

    fn new0() -> Rc<RefCell<EnvType>> {
        Rc::new(RefCell::new(EnvType::E(Env::new())))
    }

    pub fn extend(&self) -> Rc<RefCell<EnvType>> {
        let env = Env::new0();
        env.borrow_mut().set_parent(self.parent.clone());
        env
    }

    pub fn reg_procedure(&mut self, key: &str, f: fn(&mut ApplyArgs) -> LispType) {
        self.set(key, LispType::Procedure(f));
    }

    pub fn set(&mut self, key: &str, value: LispType) {
        self.env.insert(key.to_string(), value);
    }

    pub fn get(&self, key: &str) -> Option<LispType> {
        if self.env.contains_key(key) {
            Some(self.env[key].clone())
        } else {
            self.parent.borrow_mut().env(key)
        }
    }

    pub fn set_parent(&mut self, parent: Rc<RefCell<EnvType>>) {
        self.parent = parent;
    }
}
