use std::fmt::{Display, Formatter,Result};

use super::{LispType,ref_::{self, IRef, RefOps, RefRead, RefWrite}};

pub struct ConsBox {
    car: IRef<LispType>,
    cdr: IRef<LispType>,
}

impl ConsBox {
    pub fn new(car: LispType, cdr: LispType) -> Self {
        ConsBox {
            car: ref_::new(car),
            cdr: ref_::new(cdr),
        }
    }
    pub fn car(&self) -> LispType {
        self.car.ref4read().clone()
    }
    pub fn cdr(&self) -> LispType {
        self.cdr.ref4read().clone()
    }
    pub fn set_car(&self, car: LispType) {
        *self.car.ref4write() = car;
    }
    pub fn set_cdr(&self, cdr: LispType) {
        *self.cdr.ref4write() = cdr;
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
        write!(f, "({} {})", self.car.ref4read(), self.cdr.ref4read())
    }
}

impl PartialEq for ConsBox {
    fn eq(&self, other: &ConsBox) -> bool {
        self.car.ref4read().eq(&other.car.ref4read()) && self.cdr.ref4read().eq(&other.cdr.ref4read())
    }
}
