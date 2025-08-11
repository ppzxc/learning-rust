fn main() {
    string();
    memory_and_allocation();
}

fn string() {
    // let s = String::from("hello");
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{s}"); // this will print `hello, world!`
}

fn memory_and_allocation() {
    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    let s2 = s1;
}