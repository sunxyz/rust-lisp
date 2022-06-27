use crate::t::LispType;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

pub type RefEnv = Arc<RwLock<Env>>;

pub enum Env {
    Empty,
    Extend {
        parent: RefEnv,
        env: HashMap<String, LispType>,
    },
}

pub trait EnvOps {
    fn root() -> RefEnv;
    fn extend(parent: RefEnv) -> RefEnv;
    fn get(&self, key: &str) -> Option<LispType>;
    fn set(&mut self, key: &str, value: LispType);
    fn define(&mut self, key: &str, value: LispType);
}

impl EnvOps for Env {
    fn root() -> RefEnv {
        ref_env_of(Env::Extend {
            parent: ref_env_of(Env::Empty),
            env: HashMap::new(),
        })
    }

    fn extend(parent: RefEnv) -> RefEnv {
        ref_env_of(Env::Extend {
            parent: parent.clone(),
            env: HashMap::new(),
        })
    }

    fn get(&self, key: &str) -> Option<LispType> {
        match self {
            Env::Empty => None,
            Env::Extend { parent, env, .. } => {
                if let Some(v) = env.get(key) {
                    Some(v.clone())
                } else {
                    parent.read().expect("locked error").get(key).clone()
                }
            }
        }
    }

    fn set(&mut self, key: &str, value: LispType) {
        match self {
            Env::Empty => panic!("set: undefined key: {}", key),
            Env::Extend { parent, env, .. } => {
                if(env.contains_key(key)){
                    env.insert(key.to_string(), value);
                } else{
                    parent.try_write().expect("locked error").set(key, value);
                }
            }
        }
    }

    fn define(&mut self, key: &str, value: LispType) {
        match self {
            Env::Empty => panic!("define: empty env"),
            Env::Extend { parent, env, .. } => {
                if(env.contains_key(key)){
                   println!("\x1b[31m{} is already defined\x1b[0m", key);
                } else{
                    env.insert(key.to_string(), value);
                }
            }
        }
    }
}

fn ref_env_of(env: Env) -> RefEnv {
    Arc::new(RwLock::new(env))
}
