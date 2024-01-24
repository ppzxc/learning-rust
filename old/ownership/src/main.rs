// 1. 둘중 하나만 가질 수 있음
// 1.1. 하나의 가변 참조자
// 1.2. 임의 개수의 불변 참조자들

// 2. 참조자는 항상 유효한 값이어야 한다

fn main() {
    // --------------- varaible ownership
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{}, world!", s1); // borrow // error

    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);


    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    // --------------- reference, borrowing
    // & ampersand is reference
    // & ampersand parameter is borrowing
    let s1 = String::from("hello");
    let len = immutable_reference_calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // --------------- dangling reference
    let reference_to_nothing = dangle();
}

fn immutable_reference_calculate_length(s: &String) -> usize {
    s.len() // s is no dropped
} // s is no dropped

fn mutable_reference_calculate_length(s: &mut String) -> usize {
    s.push_str(", word").len() // s is no dropped
} // s is no dropped

fn dangle() -> &String {
    let s = String::from("hello");
    &s
} // s is dropped, &s is dangling reference

fn no_dangle() -> String {
    let s = String::from("hello");
    s
} // s is dropped, &s is dangling reference