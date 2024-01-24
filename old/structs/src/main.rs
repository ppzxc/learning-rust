struct User {
    email: String,
    username: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // ---------- struct init case
    let mut user_1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someuser1234"),
        sign_in_count: 1,
        active: true,
    };

    user_1.email = String::from("cjh8487@naver.com");

    // re format struct
    let user_2 = User {
        email: String::from("2@example.com"),
        username: String::from("12314214"),
        ..user_1
    };

    // tuple struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // ---------- struct use case, pixel
    let length1 = 50;
    let width1 = 30;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(length1, width1)
    );

    // use tuple
    let rect1 = (50, 30);
    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1)
    );

    // use struct
    let rect2 = Rectangle { length: 50, width: 30 };
    println!(
        "The area of the rectangle is {} square pixels.",
        area3(rect2)
    );

    let rect3 = Rectangle { length: 50, width: 30 };
    println!("{:?}", rect3); // invalid toString, error occurred
    println!("{:#?}", rect3); // invalid toString, error occurred

    // use method
    let rect4 = Rectangle { length: 50, width: 30 };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect4.area()
    );

    // can hold method
    let rect1 = Rectangle { length: 50, width: 30 };
    let rect2 = Rectangle { length: 40, width: 10 };
    let rect3 = Rectangle { length: 45, width: 60 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // no self
    let rect5 = Rectangle::square(10);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect5.area()
    );
}

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}

fn area3(rectangle: Rectangle) -> u32 {
    rectangle.length * rectangle.width
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area(length: u32, width: u32) -> u32 {
    length * width
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}