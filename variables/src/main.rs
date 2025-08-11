// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

use std::io;

fn main() {
    immutable_versus_mutable();
    shadowing();
    scala_type();
    array();
}

// immutable vs mutable
fn immutable_versus_mutable() {
    // let x = 5;
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

fn shadowing() {
    // example 1
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    };

    println!("The value of x is: {x}");

    // example 2
    // let space = "      ";
    // let space = space.len();

    // example 3
    // let mut space = "        ";
    // space = space.len();
}

fn scala_type() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // bool
    let t = true;
    let f: bool = false;

    // char
    let c = 'z';
    let c: char = 'Z';
    let heart_eyed_cat = '😻';

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}

fn array() {
    let a = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
}

fn array_in_wrong_index() {
        let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
