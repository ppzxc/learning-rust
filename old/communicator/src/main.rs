extern crate communicator;

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

// use a::series::of;
use a::series::of::nested_modules;

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{Red, Yellow};

enum TrafficLight2 {
    Red,
    Yellow,
    Green,
}

use TrafficLight2::*;

fn main() {
    communicator::client::connect();
    communicator::network::connect();
    communicator::network::server::connect();

    // a::series::of::nested_modules(); // to many name
    // of::nested_modules();

    nested_modules();

    // -------- no use
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;

    // ---------- all use
    let red = Red;
    let yellow = Yellow;
    let green = Green;
}