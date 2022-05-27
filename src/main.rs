#![allow(unused_imports)]
#![allow(unreachable_patterns)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
#![allow(dead_code)]
#![allow(type_alias_bounds)]
#![allow(unconditional_recursion)]
#![allow(mutable_borrow_reservation_conflict)]
// #![allow(non_snake_case)]

mod env;
mod interpreter;
mod parse;
mod procedure;
mod t;
use interpreter::eval;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
fn main() {
    // let s = Rc::new(RefCell::new("我很善变，还拥有多个主人".to_string()));

    // let s1 = s.clone();
    // let s2 = s.clone();
    // let s3 = Some(s2);
    // // let mut s2 = s.borrow_mut();
    // let t = s3.unwrap();
    // // s2.borrow_mut().push_str("，我很善变，还拥有多个主人");
    // t.borrow_mut().push_str(", on yeah!");
    // t.borrow_mut().push_str(", on yeah!");
    // s1.borrow_mut().push_str(", on yeah!");
    // s1.borrow_mut().push_str(", on yeah!");
    // s1.borrow_mut().push_str(", on yeah!");
    // println!("{:?}\n{:?}\n{:?}", s, s1, t);

    // println!("Hello, world!");
    // t::t();
    // print!("{}",parse::parse("((+ 1 2 3 5 6 7  (15) (- 0 1 2 3 5 6 7 (10))))").unwrap());
    // let r = parse::parse("(+ 1 2 3  7 8 9  5 6 7 (10))").unwrap();
    // println!("{}", r);
    // println!("{}", r.cdr());

    let r = eval("( (define a (lambda (x) (+ x 8))) (a 8))");
    println!("{}", r.ok().unwrap());

    // let mut j = [1] ;
    // let k = j;
    // j[0] = 2;
    // println!("{ } ",  r);
    // for i in k {
    //     println!("{}", i);
    // }
    // interpreter::print_type_of(r);
    // interpreter::interpreter(r);
    // - 1 2 3
    // - - 4 5 6
    // - 7 8 9
    // - - 10
    // let mut ts = String::from("sss");
    // ts = t(ts);
    // println!("{}", ts);
    // let tr = &mut ts[..];
    // rs(tr);
    // println!("{}", tr);
}