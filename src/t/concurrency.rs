use std::fmt::Display;
use std::sync::Barrier;
use std::sync::RwLock;
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;

use super::LispType;


pub enum ConcurrencyBox {
    THREAD(Arc<thread::JoinHandle<LispType>>),
    LOCK(Arc<Mutex<LispType>>),
    BARRIER(Arc<RwLock<Barrier>>),
}

impl Clone for ConcurrencyBox {
    fn clone(&self) -> Self {
        match self {
            ConcurrencyBox::THREAD(t) => ConcurrencyBox::THREAD(t.clone()),
            ConcurrencyBox::LOCK(l) => ConcurrencyBox::LOCK(l.clone()),
            ConcurrencyBox::BARRIER(b) => ConcurrencyBox::BARRIER(b.clone()),
        }
    }
}
    

impl Display for ConcurrencyBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConcurrencyBox::THREAD(t) => write!(f, "{}", "<THREAD>"),
            ConcurrencyBox::LOCK(l) => write!(f, "{}", "<LOCK>"),
            ConcurrencyBox::BARRIER(b) => write!(f, "{}", "<BARRIER>"),
        }
    }
}

impl  PartialEq for ConcurrencyBox {
    fn eq(&self, other: &Self) -> bool {
        match self {
            ConcurrencyBox::THREAD(t) => match other {
                ConcurrencyBox::THREAD(t2) => t.as_ref() as *const _ == t2.as_ref() as *const _,
                _ => false,
            },
            ConcurrencyBox::LOCK(l) => match other {
                ConcurrencyBox::LOCK(l2) => l.as_ref() as *const _ == l2.as_ref() as *const _,
                _ => false,
            },
            ConcurrencyBox::BARRIER(b) => match other {
                ConcurrencyBox::BARRIER(b2) => b.as_ref() as *const _ == b2.as_ref() as *const _,
                _ => false,
            },
        }
    }
}