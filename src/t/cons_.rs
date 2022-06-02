use super::*;
pub struct Cons_ {
    car: Rc<RefCell<LispType>>,
    cdr: Rc<RefCell<LispType>>,
}

impl Cons_ {
    pub fn new(car: LispType, cdr: LispType) -> LispType {
        LispType::Cons(Cons_ {
            car: Rc::new(RefCell::new(car)),
            cdr: Rc::new(RefCell::new(cdr)),
        })
    }
    pub fn car(&self) -> LispType {
        self.car.borrow().clone()
    }
    pub fn cdr(&self) -> LispType {
        self.cdr.borrow().clone()
    }
    pub fn set_car(&self, car: LispType) {
        *self.car.borrow_mut() = car;
    }
    pub fn set_cdr(&self, cdr: LispType) {
        *self.cdr.borrow_mut() = cdr;
    }
}

impl Clone for Cons_ {
    fn clone(&self) -> Self {
        Cons_ {
            car: self.car.clone(),
            cdr: self.cdr.clone(),
        }
    }
}

impl Display for Cons_ {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({} {})", self.car.borrow(), self.cdr.borrow())
    }
}

impl PartialEq for Cons_ {
    fn eq(&self, other: &Cons_) -> bool {
        self.car.borrow().eq(&other.car.borrow()) && self.cdr.borrow().eq(&other.cdr.borrow())
    }
}
