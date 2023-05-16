enum IpAddress {
    V4(String),
    V6(String),
}

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor (i32, i32, i32)
// }

// impl Message {
//     fn call(&self) -> &Self {
//         self
//     }
// }

#[allow(unused)]
fn main() {
    let home = IpAddress::V4(String::from("127.0.0.1"));
    let loopback = IpAddress::V6(String::from("::1"));
    let some_number = Some(32);
    let some_char = Some('a');
    let absent_number: Option<i32> = None;
}
