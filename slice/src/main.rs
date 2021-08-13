fn main() {
    // --------------- index, error
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();
    println!("{} {}", s, word);

    // --------------- use slice
    let he = String::from("Hello World");
    let hello = &he[0..5];  // same
    let hello = &he[..5];   // same

    let len = he.len();
    let world = &he[6..11];  // same
    let world = &he[6..len]; // same
    let world = &he[6..];    // same

    // --------------- use slice, error occurred
    // let mut s = String::from("hello world");
    // let word = first_word_use_slice(&s);
    // s.clear(); // error, slice reference
    // println!("{} {}", s, word);

    // --------------- perfect
    let my_string = String::from("hello world");
    let word = first_word_perfect(&my_string[..]);
    let my_string_literal = "hello world";
    let word = first_word_perfect(&my_string_literal[..]);
    let word = first_word_perfect(my_string_literal);
}

fn first_word_perfect(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]   // same
            // return &s[..i] // same
        }
    }

    &s[..] // no space, all word
}

fn first_word_use_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]   // same
            // return &s[..i] // same
        }
    }

    &s[..] // no space, all word
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}