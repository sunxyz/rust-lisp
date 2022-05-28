use super::t::*;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

// 单线程版本
pub struct Env(Vec<HashMap<String, LispType>>);

pub trait EnvOps {
    fn new() -> Self;
    fn get(&self, key: &str) -> Option<LispType>;
    fn set(&mut self, key: &str, value: LispType);
    fn define(&mut self, key: &str, value: LispType);
    fn fork(&mut self);
    fn kill(&mut self);
}

impl EnvOps for Env {
    
    fn new() -> Self {
        Env(vec![HashMap::new()])
    }

    fn get(&self, key: &str) -> Option<LispType> {
        println!("get key: {}, env deep: {}", key, self.0.len());
        for map in self.0.iter().rev() {
            if let Some(value) = map.get(key) {
                return Some(value.clone());
            }
        }
        None
    }

    fn set(&mut self, key: &str, value: LispType) {
        for map in &mut self.0 {
            if map.contains_key(key) {
                map.insert(key.to_string(), value.clone());
                return;
            }
        }
        panic!("{} is not defined", key);
    }

    fn define(&mut self, key: &str, value: LispType) {
        let has_define = self.0.last_mut().unwrap().contains_key(key);
        if (has_define) {
            panic!("{} is already defined", key);
        } else {
            self.0
                .last_mut()
                .unwrap()
                .insert(key.to_string(), value.clone());
        }
    }

    fn fork(&mut self) {
        self.0.push(HashMap::new());
    }

    fn kill(&mut self) {
        self.0.pop();
    }
}

impl Display for Env {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Env {}", self.0.len())
    }
}