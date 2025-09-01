enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    push();
    third();
    does_not();
    error();
    for_vector();

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

}

fn for_vector() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
}

fn error() {
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("The first element is: {first}");
}

fn does_not() {
    // let v = vec![1, 2, 3, 4, 5];
    // let does_not_exist = &v[100];
    // let does_not_exist = v.get(100);
}

fn push() {
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];
    let mut v3: Vec<i32> = Vec::new();
    v3.push(4);
    v3.push(5);
    v3.push(6);
}

fn third() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {third}");
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}
