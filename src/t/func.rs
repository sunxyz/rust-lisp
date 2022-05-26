
use crate::env::Env;
use super::*;

pub struct ApplyArgs<'a> {
    expr: List,
    args: Option<List>,
    lazy_args: fn(List,&mut  Env) -> List,
    eval0: fn(& LispType,&mut  Env) -> LispType,
    env: &'a mut  Env,
}

impl<'a> ApplyArgs<'a> {
    pub fn new(
        expr: List,
        args: Option<List>,
        lazy_args: fn(List, &mut  Env) -> List,
        eval0:fn(&LispType,&mut  Env) -> LispType,
        env:  &'a mut Env,
    ) -> Self {
        ApplyArgs {
            expr,
            args,
            lazy_args,
            eval0,
            env,
        }
    }

    pub fn expr(&self) -> &List {
        &self.expr
    }

    pub fn args(&mut self) -> &List {
        if let None = self.args {
            let lf = self.lazy_args;
            self.args = Some(lf(self.expr().clone(), self.env));
        }
        
       self.args.as_ref().unwrap()
    }

    pub fn eval(&mut self, exp:&LispType) -> LispType {
        let e:fn(&LispType ,&mut  Env)->LispType = self.eval0;
        e(exp, self.env)
    }

    pub fn env(&mut self) -> &mut Env {
        self.env
    }

}
