#![allow(unused)]

use std::ops::Deref;

fn main() {
    // box use case 1
    let b = Box::new(5);
    println!("b = {}", b);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[test]
fn dereference_use_asterisk() {
    // dereference
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
}

#[test]
fn dereference_use_box() {
    // dereference
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
}

#[test]
fn dereference_use_mybox() {
    // dereference
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
}
