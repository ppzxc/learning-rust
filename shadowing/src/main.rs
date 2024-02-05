fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("x in inner scope is : {x}");
    }

    println!("x is: {x}");

    let spaces = "          ";
    println!("{spaces}");
    let spaces = spaces.len();
    println!("{spaces}");
}
