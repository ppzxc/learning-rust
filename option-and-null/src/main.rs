fn main() {
    let some_number: Option<u8> = Some(5);
    let some_char: Option<char> = Some('e');
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y.unwrap_or_default();
}
