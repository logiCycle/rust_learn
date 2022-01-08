// use crate::List::{Cons, Nil};
use std::ops::Deref;
fn main() {
    /*     let b = Box::new(5);
    println!("{}",b);
    let mut a = Vec::new();
    a.push("2");
    println!("{:?}", a); */

/*     let list = Cons(1,
         Box::new(Cons(2,
            Box::new(Cons(3,
                 Box::new(Nil)))))); */
/*     let x = 6;
    let y = MyBox::new(x);

    assert_eq!(x, 6);
    assert_eq!(*y, 6); */
    let m = MyBox::new(String::from("Rust"));
    // &m &MyBox<String>
    //deref &String
    //deref &str
    hello(&m);
    hello(&(*m)[..]);
    hello("Rust_str");
}

fn hello(name: &str) {
    println!("Hello, {}", name);

}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
