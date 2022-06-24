use super::*;
pub struct ConsBox {
    car: Rc<RefCell<LispType>>,
    cdr: Rc<RefCell<LispType>>,
}

impl ConsBox {
    pub fn new(car: LispType, cdr: LispType) -> Self {
        ConsBox {
            car: Rc::new(RefCell::new(car)),
            cdr: Rc::new(RefCell::new(cdr)),
        }
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

impl Clone for ConsBox {
    fn clone(&self) -> Self {
        ConsBox {
            car: self.car.clone(),
            cdr: self.cdr.clone(),
        }
    }
}

impl Display for ConsBox {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({} {})", self.car.borrow(), self.cdr.borrow())
    }
}

impl PartialEq for ConsBox {
    fn eq(&self, other: &ConsBox) -> bool {
        self.car.borrow().eq(&other.car.borrow()) && self.cdr.borrow().eq(&other.cdr.borrow())
    }
}
