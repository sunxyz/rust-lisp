// Cons list vector dict set
use super::LispType;
use std::cell::RefCell;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::rc::Rc;

enum ListType {
    SUB,
    EXPR,
}

pub struct List(Rc<RefCell<Vec<LispType>>>, ListType);

impl List {
    pub fn new() -> Self {
        List(Rc::new(RefCell::new(Vec::new())), ListType::EXPR)
    }

    pub fn of(elem_s: Vec<LispType>) -> Self {
        List(Rc::new(RefCell::new(elem_s)), ListType::EXPR)
    }

    pub fn car(&self) -> LispType {
        self.0.borrow()[0].clone()
    }

    pub fn cdr(&self) -> List {
        let t = self.0.borrow()[1..].to_vec();
        List(Rc::new(RefCell::new(t)), ListType::SUB)
    }

    pub fn is_nil(&self) -> bool {
        self.0.borrow().len() == 0
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

    pub fn push(&mut self, elem: LispType) {
        self.0.borrow_mut().push(elem);
    }

    pub fn push_all(&mut self, elem: Vec<LispType>) {
        self.0.borrow_mut().extend(elem);
    }

    pub fn data(&self) -> &Vec<LispType> {
        &self.0.borrow()
    }
}

impl Copy for ListType {}
impl Clone for ListType {
    fn clone(&self) -> Self {
        match self {
            ListType::EXPR => ListType::EXPR,
            ListType::SUB => ListType::SUB,
        }
    }
}

impl Display for List {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "({})",
            self.0
                .borrow()
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl Clone for List {
    fn clone(&self) -> Self {
        List(self.0.clone(), self.1)
    }
}
