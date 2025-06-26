// enum IpAddrKind {
//     V4(u8, u8, u8, u8 ),
//     V6(String ),
// }
//
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }
//
// enum Message {
//     Quit,
//     Move { x: i32, y: i32},
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// impl Message {
//     fn hello(){
//         println!("Hello, world!");
//     }
// }
// fn main() {
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;
//
//     let localhost = IpAddrKind::V4(127,0,0,1 );
//
// }
// fn route(ip_kind: IpAddrKind) {
//
// }

// fn main(){
//     // enum Option<T> {
//     //     Some(T),
//     //     None,
//     // }
//
//     let x: i8 = 5;
//     let y: Option<i8> = None;
//
//     let sum = x + y.unwrap_or(0) ;
// }

fn main(){
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x{
        Some(i) => Some(i + 1),
        _ => (),
    }
}