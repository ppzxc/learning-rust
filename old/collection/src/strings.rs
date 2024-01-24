#![allow(unused)]

use std::ops::Add;

pub fn string_create() {
    println!("Hello Strings...");

    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contetns".to_string();
    let s = String::from("initial contents");

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}

pub fn string_renewal() {
    // case 1
    let mut s = String::from("foo");
    s.push_str("bar");

    // case 2
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(&s2);
    println!("s2 is {}", s2);

    // case 3
    let mut s = String::from("lo");
    s.push('l');
    println!("s is {}", s);

    // case 4
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    // s4 = s1.add(&s2);
    // println!("{} {} {} {}", s1, s2, s3, s4);

    // case 5
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s = s1 + "-" + &s2 + "-" + &s3; // use uncomfortable
    let s = format!("{}-{}-{}", s1, s2, s3); // comfortable
}

pub fn string_iter() {
    // String == Vec<u8>    -> UTF8 character
    println!("LEN IS {}", String::from("Hola").len());
    println!("LEN IS {}", String::from("Здравствуйте").len());
    println!("LEN IS {}", String::from("안녕하세요").len());
}

pub fn string_utf8_char() {
    // hindi        “नमस्ते”
    // bytes        [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    // char         ['न', 'म', 'स', '्', 'त', 'े']
    // char cluster ["न", "म", "स्", "ते"]

    let hello = "Здравствуйте";
    // let s = &hello[0..1]; // error!!!!!!!!!!!!!!

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
}