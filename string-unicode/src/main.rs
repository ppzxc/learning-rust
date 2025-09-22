fn main() {
    different_string();
    unicode_String();
    add_string();
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push(s2);
    println!("s2 is {s2}");
}

fn add_string() {
    let mut s = String::from("foo");
    s.push_str("bar");
}

fn unicode_String() {
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}

fn different_string() {
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");
}
