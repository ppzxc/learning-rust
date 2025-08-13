fn main() {
    let reference = dangle();
    println!("{reference}");
}

// NOT WORKING
// fn dangle() -> &String {
//     let s = String::from("hello");
//
//     &s
// }

fn dangle() -> String {
    let s = String::from("hello");

    s
}