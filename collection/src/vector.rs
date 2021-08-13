#![allow(unused)]
pub fn vector_1() {
    println!("vector chapter 1");

    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);

    println!("{:?}", v);

    // ----------- access vector
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("{:?}", third);
    let third: Option<&i32> = v.get(2);
    println!("{:?}", third);

    let v = vec![1, 2, 3, 4, 5];
    // let does_not_exist = &v[100];        // index out of range, panic!
    // println!("{:?}", does_not_exist);    // index out of range, panic!
    let does_not_exist = v.get(100); // out of range return none
    println!("{:?}", does_not_exist);

    // ------------ reference, push
    let mut v2 = vec![1, 2, 3, 4, 5];
    let first = &v2[0];
    v2.push(6);

    // ------------- iterator
    let v3 = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v4 = vec![100, 32, 57];
    for i in &mut v4 {
        *i += 50;
    }
    println!("{:?}", v4);

    // ---------------- vector parameter
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

fn vector_drop() {
    let v = vec![1, 2, 3, 4];
    // something do
} // v drop