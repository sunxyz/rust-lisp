use super::*;
use crate::env::Env;
use crate::t::LispType::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct ApplyArgs<'a> {
    expr: List,
    args: Option<List>,
    lazy_args: fn(List, &mut Env) -> List,
    inter: fn(&LispType, &mut Env) -> LispType,
    env: &'a mut Env,
}

impl<'a> ApplyArgs<'a> {
    pub fn new(
        expr: List,
        args: Option<List>,
        lazy_args: fn(List, &mut Env) -> List,
        inter: fn(&LispType, &mut Env) -> LispType,
        env: &'a mut Env,
    ) -> Self {
        ApplyArgs {
            expr,
            args,
            lazy_args,
            inter,
            env,
        }
    }

    pub fn expr(&self) -> &List {
        &self.expr
    }

    pub fn args(&mut self) -> &List {
        if let None = self.args {
            let lazy_f = self.lazy_args;
            let v = lazy_f(self.expr().clone(), self.env);
            println!("args is None exp: {} => {}", self.expr(), v);
            self.args = Some(v);
        }
        self.args.as_ref().unwrap()
    }

    pub fn inter(&mut self, exp: &LispType) -> LispType {
        let e: fn(&LispType, &mut Env) -> LispType = self.inter;
        e(exp, self.env)
    }

    pub fn inter_4_env(&mut self, exp: &LispType, env: &mut Env) -> LispType {
        let e: fn(&LispType, &mut Env) -> LispType = self.inter;
        e(exp, env)
    }

    pub fn env(&mut self) -> &mut Env {
        self.env
    }

    pub fn apply(&mut self) -> LispType {
        let args = self.args();
        if let Procedure(f) = args.car() {
            let args = args.cdr();
            if let Some(Expr(last)) = args.data().last() {
                let a = last.data().to_vec();
                let mut args = List::new();
                args.push_all(a);
                println!("apply: {}", args);
                args.push(Expr(last.clone()));
                self.args = Some(args);
            }else {
                panic!("apply: invalid last argument");
            }
            f(self)
        } else {
            panic!("apply: invalid argument");
        }
    }
}
