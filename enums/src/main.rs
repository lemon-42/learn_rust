#[allow(unused_variables)]

// An enum defines a type by enumerating its possible variants. Each variant can optionally hold data.
// Here, IpAddrKind can be either V4 (which holds four u8 values) or V6 (which holds a String).
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Another example of an enum, where each variant represents a different kind of message.
// Variants may or may not contain associated data.
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

//fn router(kind: IpAddrKind) {}

fn main() {
    // Create a instance of each of the two variants
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    //router(IpAddrKind::V4);
    //router(IpAddrKind::V6);

    let local = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();
}
