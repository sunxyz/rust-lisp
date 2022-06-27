// Cons list vector dict set
use super::LispType;
use std::cell::RefCell;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::RwLock;

enum ListType {
    SUB,
    EXPR,
}

pub struct List(Arc<RwLock<Vec<LispType>>>, ListType);

impl List {
    pub fn new() -> Self {
        List(Arc::new(RwLock::new(Vec::new())), ListType::EXPR)
    }

    pub fn of(elem_s: Vec<LispType>) -> Self {
        List(Arc::new(RwLock::new(elem_s)), ListType::EXPR)
    }

    pub fn car(&self) -> LispType {
        self.0.try_read().expect("locked car")[0].clone()
    }

    

    pub fn cdr(&self) -> List {
        let t = self.0.try_read().expect("locked list")[1..].to_vec();
        List(Arc::new(RwLock::new(t)), ListType::SUB)
    }

    pub fn is_nil(&self) -> bool {
        self.0.try_read().expect("locked list").len() == 0
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
        self.0.try_write().expect("locked list").push(elem);
    }

    pub fn push_vec(&mut self, elem: Vec<LispType>) {
        self.0.try_write().expect("locked list").extend(elem);
    }

    pub fn push_all(&mut self, elem: List) {
        self.0.try_write().expect("locked list").extend(elem.data());
    }

    pub fn len(&self) -> usize {
        self.0.try_read().expect("locked list").len()
    }

    pub fn data(&self) -> Vec<LispType> {
        self.0.try_read().expect("locked list").clone()
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
                .try_read().expect("locked list")
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

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        if self.0.try_read().expect("locked list").len() != other.0.try_read().expect("locked list").len() {
            return false;
        }
        for i in 0..self.0.try_read().expect("locked list").len() {
            if self.0.try_read().expect("locked list")[i] != other.0.try_read().expect("locked list")[i] {
                return false;
            }
        }
        true
    }
}

// impl Iterator for List {
//     type Item = LispType;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.is_nil() {
//             None
//         } else {
//             let t = self.0.borrow_mut().remove(0);
//             Some(t)
//         }
//     }
// }