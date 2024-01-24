fn main() {
    // --------- function
    println!("Hello, world!");
    another_function();
    another_function_parameter(5);
    another_function_parameter_two(5, 10);

    // ---------- expression
    // let x = (let y = 6); // error
    let x = 8;
    let y = {
        let x = 3;
        x + 1
    };
    println!("the value of before x, y is : {} {}", x, y);

    // ---------- return function
    let x = five();
    println!("the value of x is : {}", x);

    let x = plus_one(10);
    println!("the value of x is : {}", x);

    // ---------- comment
    // comment1
    // comment2
}

fn another_function() {
    println!("Another Function.");
}

fn another_function_parameter(x: i32) {
    println!("Another Function parameter = {}", x);
}

fn another_function_parameter_two(x: i32, y: i32) {
    println!("Another Function parameter two = {}, {}", x, y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}