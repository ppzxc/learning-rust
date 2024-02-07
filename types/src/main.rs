fn main() {
    // type
    // let guess: u32 = "42".parse().expect("Not A Number!");

    //
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

    // remainder
    let remainder = 43 % 5;

    // bool
    let t = true;
    let f: bool = false;

    // char
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_car = '😻';

    // compound type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup2 = (500, 6.4, 1);
    let (x, y, z) = tup2;
    println!("The Value Of Y IS :{tup}");
    println!("The Value Of Y IS :{tup2}");
    println!("The Value Of Y IS :{x} {y} {z}")
}

