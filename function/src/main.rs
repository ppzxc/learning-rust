fn main() {
    println!("Hello, world!");

    another_function();
    second_another_function(5);
    print_labeled_measurement(5, 'h');
    statements_and_expressions();
    return_functions();
}

fn another_function() {
    println!("Another function");
}

fn second_another_function(x: i32) {
    println!("Second Another function: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn statements_and_expressions() {
    let y1 = 6;
    // let x1 = (let y2 = 6);
    let y2 = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y2}")
}

fn return_functions() {
    let x = five();
    println!("The value of x is: {x}");
    let x2 = plus_one(5);
    println!("The value of x is: {x2}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}