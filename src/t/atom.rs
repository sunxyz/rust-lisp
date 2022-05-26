use super::LispType;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
// boolean number char
// pub trait Atom: LispType {}
// pub trait New {
//      fn new(s: &str) -> Self;
// }
// pub type Number = i32;
// pub type Char = char;
// pub type Boolean = bool;

// pub struct Nil ();

// pub static TRUE: Boolean = true;
// pub static FALSE: Boolean = false;

// impl New for i32 {
//      fn new(s: &str) -> Self {
//         s.parse::<i32>().unwrap()
//     }
// }
// impl New for char {
//      fn new(s: &str) -> Self {
//         s.chars().nth(1).unwrap()
//     }
// }

// impl New for bool {
//      fn new(s: &str) -> Self {
//         if(s == "#t") {
//             TRUE
//         } else {
//             FALSE
//         }
//     }
// }

// impl Atom for i32 {}
// impl Atom for char {}
// impl Atom for bool {}
// impl Atom for Nil {}
    
// impl LispType for i32 {}
// impl LispType for char {}
// impl LispType for bool {}
// impl LispType for Nil {}
// impl Display for Nil {
//     fn fmt(&self, f: &mut Formatter) -> Result {
//         write!(f, "nil")
//     }
// }
   
// pub fn t(){
//     let n = Number::new("1");
//     let t :Box<dyn LispType> = Box::new(n);
//     println!("{}", t);
// }