// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

#[derive(Debug)]
struct Area {
    width: u32,
    height:u32,
}

impl Area {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Area) -> bool {
        self.width > other.width && self.height > other.height
    }
    
}
impl Area{
    fn square(size: u32) -> Area {
        Area {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // let mut user1 = User {
    //     email: String::from("tadylucas@root.rocs"),
    //     username: String::from("tadylucas"),
    //     active: true,
    //     sign_in_count: 2,
    // };
    //
    // let name = user1.username;
    // user1.username = String::from("root");
    //
    // let user2 = build_user(String::from("john@doe.com"), String::from("JohnDoe"));
    //
    // let user3 = User{
    //     email: String::from("james@mail.com"),
    //     username: String::from("james"),
    //     ..user2
    // };


    // struct Color(i32, i32, i32);
    // struct Point(i32, i32, i32);

    let rectangle = Area {
        height: 50,
        width: 50,
    };

    let rect1 = Area{
        width: 20,
        height: 40,
    };

    let rect2 = Area{
        width:40,
        height: 50,
    };
    let rect3 = Area::square(20);

    println!("rect can hold rect1: {}", rectangle.can_hold(&rect1));
    println!("rect can hold rect2: {}", rectangle.can_hold(&rect2));

    println!("rect: {:#?}", rectangle);
    println!("The area of the rectangle is {} square pixels.", rectangle.area());

}

// fn build_user(email: String, username: String) -> User{
//     User {
//         email,
//         username,
//         sign_in_count: 1,
//         active: true,
//     }
// }
