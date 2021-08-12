const MAX_POINTS: u32 = 100_000;

fn main() {
    // --------- mutable
    let mut x = 5;
    println!("The value of x is : {}", x);
    x = 6;
    println!("The value of x is : {}", x);

    // --------- immutable
    let y =75;
    println!("The value of x is : {}", y);
    // y = 6; //error
    // println!("The value of x is : {}", y); // error

    // --------- constant
    println!("The value of MAX_POINTS is : {}", MAX_POINTS);

    // --------- immutable shadowing
    let z = 5;
    let z = z + 1;
    let z = z * 2;
    println!("The value of z is: {}", z);

    // --------- shadowing vs mutable
    // possible case
    let spaces_possible = "   ";
    let spaces_possible = spaces_possible.len();
    println!("The value of spaces_possible is: {}", spaces_possible);

    // impossible case
    // let mut spaces_impossible = "   ";
    // spaces_impossible = spaces_impossible.len(); // error

    // --------- types
    // let guess = "42".parse().expect("Not a number!"); // error
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {}", guess);

    // in rust types
    //  length    8,  16,  32,  64,  arch // bit
    // singned   i8, i16, i32, i64, isize // signed
    // unsingned u8, u16, u32, u64, usize // unsigned

    // normal use case, fastest i32

    let xx = 2.0; // f64
    println!("The value of xx is: {}", xx);
    let yy: f32 = 3.0; // f32
    println!("The value of yy is: {}", yy);

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    println!("operation: {} {} {} {} {}", sum, difference, product, quotient, remainder);

    // boolean
    let t = true;
    let f: bool = false;

    println!("booleans {} {}", t, f);

    // character
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("character {} {} {}", c, z, heart_eyed_cat);

    // --------- tuple
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("tuple {} {} {}", x, y, z);

    // pointer
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("tuple pointer {} {} {}", five_hundred, six_point_four, one);

    // --------- array
    let a = [1, 2, 3, 4, 5];
    // let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    let first = a[0];
    let second = a[1];
    println!("array 1 = {} {}", first, second);

    // let index = 10;
    // println!("panic {}", a[index]);
}
