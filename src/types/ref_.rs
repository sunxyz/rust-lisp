use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub type IRef<T: ?Sized> = Arc<RwLock<T>>;
pub type RefRead<'a, T: ?Sized> = RwLockReadGuard<'a, T>;
pub type RefWrite<'a, T: ?Sized> = RwLockWriteGuard<'a, T>;

pub trait RefOps<T>: Clone {
    fn ref4read(&self) -> RefRead<'_, T>;
    fn ref4write(&self) -> RefWrite<'_, T>;
    fn into_inner(self) -> T;
}

impl<T> RefOps<T> for IRef<T> {
    fn ref4read(&self) -> RefRead<'_, T> {
        self.read().expect("get_read: poisoned")
    }
    fn ref4write(&self) -> RefWrite<'_, T> {
        self.write().expect("get_write: poisoned")
    }
    fn into_inner(self) -> T {
        self.into_inner()
    }
}

pub fn new<V>(t: V) -> IRef<V> {
    Arc::new(RwLock::new(t))
}
