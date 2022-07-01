use super::{LispType,ref_::{self, IRef, RefOps, RefRead, RefWrite}};
// Cons list vector dict set
use std::fmt::{Display, Formatter,Result};

enum ListType {
    SUB,
    EXPR,
}

pub struct List(IRef<Vec<LispType>>, ListType);

impl List {
    pub fn new() -> Self {
        List(ref_::new(Vec::new()), ListType::EXPR)
    }

    pub fn of(elem_s: Vec<LispType>) -> Self {
        List(ref_::new(elem_s), ListType::EXPR)
    }

    pub fn car(&self) -> LispType {
        self.0.ref4read()[0].clone()
    }

    pub fn cdr(&self) -> List {
        let t = self.0.ref4read()[1..].to_vec();
        List(ref_::new(t), ListType::SUB)
    }

    pub fn is_nil(&self) -> bool {
        self.0.ref4read().len() == 0
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
        self.0.ref4write().push(elem);
    }

    pub fn push_vec(&mut self, elem: Vec<LispType>) {
        self.0.ref4write().extend(elem);
    }

    pub fn push_all(&mut self, elem: List) {
        self.0.ref4write().extend(elem.data());
    }

    pub fn len(&self) -> usize {
        self.0.ref4read().len()
    }

    pub fn data(&self) -> Vec<LispType> {
        self.0.ref4read().clone()
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
                .ref4read()
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
        if self.0.ref4read().len() != other.0.ref4read().len() {
            return false;
        }
        for i in 0..self.0.ref4read().len() {
            if self.0.ref4read()[i] != other.0.ref4read()[i] {
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
