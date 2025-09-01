enum IpAddrKind {
    V4,
    V6,
}

struct IpAddrStructure {
    kind: IpAddrKind,
    address: String,
}

enum IpAddr1 {
    V4(String),
    V6(String),
}

enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr3 {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

fn main() {
    basic();
    enum_value();
    ip_addr_1();
    ip_addr_2();
    enum_value_2();
}

fn enum_value_2() {
    let m = Message::Write(String::from("hello"));
    m.call();
}

fn ip_addr_2() {
    let home = IpAddr2::V4(127, 0, 0, 1);
    let loopback = IpAddr2::V6(String::from("::1"));
}

fn ip_addr_1() {
    let home = IpAddr1::V4(String::from("127.0.0.1"));
    let loopback = IpAddr1::V6(String::from("::1"));
}

fn enum_value() {
    let home = IpAddrStructure {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddrStructure {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

fn basic() {
    let version_four = IpAddrKind::V4;
    let version_six = IpAddrKind::V6;
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

fn route(ip_kind: IpAddrKind) {}
