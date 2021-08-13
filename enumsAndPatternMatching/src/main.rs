enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddr2 {
    V4(String),
    V6(String),
}

enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    // --------------- default enum
    let v4 = IpAddrKind::V4;
    let v6 = IpAddrKind::V6;
    route(v4);
    route(v6);
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // --------------- default enum
    let v4_loopback = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let v6_loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // --------------- string enum
    let v4_loopback_2 = IpAddr2::V4(String::from("127.0.0.1"));
    let v6_loopback_2 = IpAddr2::V6(String::from("::1"));

    // --------------- tuple enum
    let v4_loopback_3 = IpAddr3::V4(127, 0, 0, 1);
    let v6_loopback_3 = IpAddr3::V6(String::from("::1"));

    // ------------------- option
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    // ------------------- option example
    let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    let y: Option<i8> = None;
    let sum = x + y.unwrap_or(0);

    // ------------------- match
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => {
                println!("Lucky Penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // ... ETC
    }

    enum Coin2 {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents2(coin: Coin2) -> u32 {
        match coin {
            Coin2::Penny => 1,
            Coin2::Nickel => 5,
            Coin2::Dime => 10,
            Coin2::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            },
        }
    }

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // etc whole matching
    }

    // ------------------- if let
    let some_u8_value = Some(0u8);
    match some_u8_value { // one pattern matching other etc
        Some(3) => println!("three"),
        _ => (),
    }

    // boilerplate
    if let Some(3) = some_u8_value {
        println!("three");
    }


}

fn route(ip_type: IpAddrKind) {}
