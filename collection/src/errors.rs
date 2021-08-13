#![allow(unused)]

use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

pub fn errors_learning() {
    // -------------------- panic
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];
    // v[99];

    // -------------------- recoverable panic, errors
    // let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("There was a problem opening the file : {:?}", error);
    //     },
    // };

    //

    // let f = File::open("hello.txt");
    //
    // let f = match f {
    //     Ok(file) => file,
    //     Err(ref error) if error.kind() == ErrorKind::NotFound => {
    //         match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => {
    //                 panic!(
    //                     "Tried to create file but there was a problem: {:?}",
    //                     e
    //                 )
    //             },
    //         }
    //     },
    //     Err(error) => {
    //         panic!(
    //             "There was a problem opening the file: {:?}",
    //             error
    //         )
    //     },
    // };

    // let f = File::open("hello.txt").unwrap(); // error
    // let f = File::open("hello.txt").expect("hello.txt file open failed"); // error
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?; // chaining

    Ok(s)
}