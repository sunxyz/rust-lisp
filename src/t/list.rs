// Cons list vector dict set
use super::LispType;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;


pub enum ListType {
    SUB,
    EXPR,
}
pub struct List(Vec<LispType>, ListType);



impl List {
    pub fn new() -> Self {
        List(Vec::new(), ListType::EXPR)
    }

    pub fn car(&self) -> LispType {
        self.0[0].clone()
    }

    pub fn cdr(& self) -> List {
        List(self.0[1..].to_vec(), ListType::EXPR)
    }

    pub fn is_nil(&self) -> bool {
        self.0.len() == 0
    }

    pub fn is_expr(&self) -> bool {
        if let ListType::EXPR = self.1 {
            true
        } else {
            false
        }
    }

    pub fn is_sub(&self) -> bool {
        if let ListType::SUB = self.1 {
            true
        } else {
            false
        }
    }

    pub fn push(&mut self, elem:  LispType) {
        self.0.push(elem);
    }

    pub fn push_all(&mut self, elem: Vec<LispType>) {
        self.0.extend(elem);
    }

    pub fn data(&self) -> &Vec<LispType> {
        &self.0
    }

}

impl Copy for ListType{
   
}
impl Clone for ListType{
    fn clone(&self) -> Self {
        match self {
            ListType::EXPR => ListType::EXPR,
            ListType::SUB => ListType::SUB,
        }
    }
}
    

impl  Display for List {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({})", self.0.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "))
    }
}

impl Clone for List {
    fn clone(&self) -> Self {
        let mut v = Vec::new();
        for i in self.0.iter(){
           v.push(i.clone());
        }
        List(v, self.1)
    }
}