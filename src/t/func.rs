use super::*;
use crate::env::Env;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Func {
    pub name: String,
    pub args: Option<Vec<String>>,
    pub body: Option<List>,
    pub func: Option<fn(&mut ApplyArgs) -> LispType>,
}

impl Func {
    pub fn new (name: String, args: Option<Vec<String>>, body: Option<List>, func: Option<fn(&mut ApplyArgs) -> LispType>) -> Func {
        Func {
            name,
            args,
            body,
            func
        }
    }

    pub fn call(&self, args: &mut ApplyArgs) -> LispType {
        // unimplemented!()
        if(self.func.is_some()) {
            print!("self: {}", self.name);
            (self.func.as_ref().unwrap())(args)
        } else {
            panic!("{} is not a procedure", self.name);
        }
    }
}

impl Clone for Func {
    fn clone(&self) -> Self {
        self.clone()
    }
}


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

    pub fn inter_4_env(&mut self, exp: &LispType, env: Rc<RefCell<Env>>) -> LispType {
        let e: fn(&LispType, &mut Env) -> LispType = self.inter;
        e(exp, &mut env.borrow_mut())
    }

    pub fn env(&mut self) -> &mut Env {
        self.env
    }
}
