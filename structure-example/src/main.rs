#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width = 30;
    let height = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width, height)
    );

    let rect = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_2(rect)
    );

    let rect2 = &Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area_3(rect2)
    );
    let rect3 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {rect3:?} square pixels.");
    println!("The area of the rectangle is {rect3:#?} square pixels.");

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
