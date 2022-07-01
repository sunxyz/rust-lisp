use std::{cell::{RefCell, Ref, RefMut},  rc::Rc};

pub type IRef<T : ?Sized> = Rc<RefCell<T>>;

pub trait RefOps<T>: Clone {
    fn read(&self) -> Ref<T>;
    fn write(& self) -> RefMut<T>;
    fn into_inner(self) -> T;
}

impl<T> RefOps<T> for IRef<T> {
    fn read(&self) -> Ref<T> {
         self.borrow()
    }
    fn write(&self) -> RefMut<'_, T> {
        self.borrow_mut()
    }
    fn into_inner(self) -> T {
        self.into_inner()
    }
}

pub fn new<V>(t: V)->IRef<V>{
    Rc::new(RefCell::new(t))
}


fn t(){
    let v = new("hello");
    let t = v.borrow_mut();
    println!("{}", t);
    t.chars();
}