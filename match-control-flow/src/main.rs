enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
    Yen,
    Won,
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // -- snip --
        }
    }
}

enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
    Yen,
    Won,
}

fn main() {
    println!("{}", value_in_cents(Coin::Penny));
    println!("{}", value_in_cents(Coin::Quarter));
    println!("{}", value_in_cents_2(Coin::Penny));
    println!("{}", value_in_cents_2(Coin::Quarter));
    println!("{}", value_in_cents_3(Coin2::Penny));
    println!("{}", value_in_cents_3(Coin2::Quarter(UsState::Alabama)));
    println!("{}", value_in_cents_3(Coin2::Quarter(UsState::Alaska)));

    //
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // if let else
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
    let config_max2: Option<u8> = None;
    match config_max2 {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    // if let else, same logic
    let config_max3 = Some(3u8);
    if let Some(max) = config_max3 {
        println!("The maximum is configured to be {max}");
    }

    let coin = Coin2::Quarter(UsState::Alabama);
    // using coin
    let mut count = 0;
    match coin {
        Coin2::Quarter(state) => println!("State quarter from {state:?}!"),
        _ => count += 1,
    }

    // using coin, same
    let mut count = 0;
    // if let Coin2::Quarter(state) = coin {
    //     println!("State quarter from {state:?}!");
    // } else {
    //     count += 1;
    // }
}

fn describe_state_quarter(coin: Coin2) -> Option<String> {
    // let state = if let Coin2::Quarter(state) = coin {
    //     state
    // } else {
    //     return None;
    // };

    let Coin2::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        Coin::Yen => 30,
        Coin::Won => 40,
    }
}

fn value_in_cents_2(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        Coin::Yen => 30,
        Coin::Won => 40,
    }
}

fn value_in_cents_3(coin: Coin2) -> u8 {
    match coin {
        Coin2::Penny => 1,
        Coin2::Nickel => 5,
        Coin2::Dime => 10,
        Coin2::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
        Coin2::Yen => 30,
        Coin2::Won => 40,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
