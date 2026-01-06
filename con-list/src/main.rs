enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let _b = Cons(3, Rc::clone(&a));
    // let _c = Cons(4, a.clone());
    // Rust convension is to use Rc::clone instead of cloning Rc<T>
    let _c = Cons(4, Rc::clone(&a));
}
