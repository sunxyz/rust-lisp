use std::fmt::{Display, Formatter,Result};

use std::thread;
use std::sync::{Barrier,RwLock,Arc,Mutex};

use super::LispType;


pub enum ConcurrencyBox {
    THREAD(Arc<Mutex<Option<thread::JoinHandle<LispType>>>>),
    LOCK(Arc<Mutex<LispType>>),
    BARRIER(Arc<RwLock<Barrier>>),
    Channel(flume::Sender<LispType>, flume::Receiver<LispType>),
}

impl Clone for ConcurrencyBox {
    fn clone(&self) -> Self {
        match self {
            ConcurrencyBox::THREAD(t) => ConcurrencyBox::THREAD(t.clone()),
            ConcurrencyBox::LOCK(l) => ConcurrencyBox::LOCK(l.clone()),
            ConcurrencyBox::BARRIER(b) => ConcurrencyBox::BARRIER(b.clone()),
            ConcurrencyBox::Channel(tx, rx) => ConcurrencyBox::Channel(tx.clone(), rx.clone()),
        }
    }
}
    

impl Display for ConcurrencyBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConcurrencyBox::THREAD(t) => write!(f, "{}", "<THREAD>"),
            ConcurrencyBox::LOCK(l) => write!(f, "{}", "<LOCK>"),
            ConcurrencyBox::BARRIER(b) => write!(f, "{}", "<BARRIER>"),
            ConcurrencyBox::Channel(tx, rx) => write!(f, "{}", "<CHANNEL>"),
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
            ConcurrencyBox::Channel(tx, rx) => match other {
                _ => false,
            },
        }
    }
}

// pub struct WarpBox<T>{
//     warp: Option<T>,
// }

// impl<T> WarpBox<T> {
//     fn new(t : T) -> Self{
//         WarpBox{
//             warp: Some(t),
//         }
//     }
//     fn unwrap(&mut self) -> T{
//        self.warp.take().unwrap()
//     }
       
// }