// struct User {
//     username: String,
//     email:String,
//     sign_in_count:u64,
//     active: bool,
// }


// fn main() {
//     let mut user1 = User{
//         email: String::from("test@mail.com"),
//         username: String::from("test"),
//         sign_in_count: 1,
//         active: true,
//     };
//     let name = user1.username;
//     user1.username = String::from("test");

//     let user2 = build_user(String::from("hello"), String::from("tes@mail.com"));

//     let user3  = User {
//         username: String::from("test"),
//         email: String::from("email@email.com"),
//         ..user2
//     };
// }


// fn build_user(email: String, username: String) -> User{
//     User{
//         username,
//         email,
//         sign_in_count: 1,
//         active: true,
//     }
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}


impl Rectangle {
    fn area (&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}




fn main(){

    let rectangle = Rectangle {
        width: 20,
        height: 40
    };

    let rect_1 = Rectangle {
        width: 10,
        height: 19
    };

    let rect_2 = Rectangle {
        width: 30,
        height: 60
    };

    let rect3 = Rectangle::square(12);

    println!("{}", rectangle.can_hold(&rect_1));

    println!("area og the square is{}",rectangle.area());
}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.height* rectangle.width
// }