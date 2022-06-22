use super::*;
use crate::env::Env;
use crate::env::EnvOps;
use crate::env::RefEnv;
use crate::t::LispType::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct ApplyArgs {
    expr: List,
    args: Option<List>,
    lazy_args: fn(List, RefEnv) -> List,
    inter: fn(&LispType, RefEnv) -> LispType,
    env: RefEnv,
}

impl ApplyArgs {
    pub fn new(
        expr: List,
        args: Option<List>,
        lazy_args: fn(List, RefEnv) -> List,
        inter: fn(&LispType, RefEnv) -> LispType,
        env: RefEnv,
    ) -> Self {
        ApplyArgs {
            expr,
            args,
            lazy_args,
            inter,
            env,
        }
    }

    pub fn clone_of(&mut self, args: Option<List>) -> ApplyArgs {
        ApplyArgs::new(
            if let Some (l) = args.clone() {l} else {List::new()},
            args,
            |l, v| List::new(),
            self.inter,
            self.env.clone(),
        )
    }

    pub fn fork_env(&mut self) -> ApplyArgs {
        let mut child = self.clone();
        child.env = Env::extend(self.env.clone());
        child
    }

    pub fn expr(&self) -> &List {
        &self.expr
    }

    pub fn args(&mut self) -> &List {
        if let None = self.args {
            let lazy_f = self.lazy_args;
            let v = lazy_f(self.expr().clone(), self.env.clone());
            // println!("args is None exp: {} => {}", self.expr(), v);
            self.args = Some(v);
        }
        self.args.as_ref().unwrap()
    }

    pub fn inter(&mut self, exp: &LispType) -> LispType {
        let e = self.inter;
        e(exp, self.env.clone())
    }

    pub fn inter_4_env(&mut self, exp: &LispType, env: RefEnv) -> LispType {
        let e = self.inter;
        e(exp, env)
    }

    pub fn env(&mut self) -> RefEnv {
        self.env.clone()
    }

    pub fn apply(&mut self) -> LispType {
        let args = self.args();
        if let Procedure(f) = args.car() {
            let args = args.cdr();
            if (args.is_nil()) {
                self.args = None;
            } else {
                let data = args.data();
                if let Some(Expr(last)) = data.last() {
                    let a = args.data()[0..data.len() - 1].to_vec();
                    let mut args = List::new();
                    args.push_vec(a);
                    args.push_vec(last.data().clone());
                    // println!("apply: {}", args);
                    self.args = Some(args);
                } else {
                    panic!("apply: invalid last argument");
                }
            }
            f(self)
        } else {
            panic!("apply: invalid argument");
        }
    }
}

impl Clone for ApplyArgs {
    fn clone(&self) -> Self {
        ApplyArgs::new(
            self.expr.clone(),
            self.args.clone(),
            self.lazy_args,
            self.inter,
            self.env.clone(),
        )
    }
}
