use super::*;
pub struct ConsBox {
    car: Arc<Mutex<LispType>>,
    cdr: Arc<Mutex<LispType>>,
}

impl ConsBox {
    pub fn new(car: LispType, cdr: LispType) -> Self {
        ConsBox {
            car: Arc::new(Mutex::new(car)),
            cdr: Arc::new(Mutex::new(cdr)),
        }
    }
    pub fn car(&self) -> LispType {
        self.car.try_lock().expect("locked err").clone()
    }
    pub fn cdr(&self) -> LispType {
        self.cdr.try_lock().expect("locked err").clone()
    }
    pub fn set_car(&self, car: LispType) {
        *self.car.try_lock().expect("locked err") = car;
    }
    pub fn set_cdr(&self, cdr: LispType) {
        *self.cdr.try_lock().expect("locked err") = cdr;
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
        write!(f, "({} {})", self.car.try_lock().expect("locked err"), self.cdr.try_lock().expect("locked err"))
    }
}

impl PartialEq for ConsBox {
    fn eq(&self, other: &ConsBox) -> bool {
        self.car.try_lock().expect("locked err").eq(&other.car.try_lock().expect("locked err")) && self.cdr.try_lock().expect("locked err").eq(&other.cdr.try_lock().expect("locked err"))
    }
}
