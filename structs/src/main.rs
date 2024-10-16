// struct User {
//     active: bool,
//     username: String,
//     sign_in_count: u32,
// }

// struct Coordinates(i32, i32, i32);

// struct UniStruct;

// struct Square {
//     width: u32,
//     height: u32,
// }

// impl Square {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn whats_my_width(&self) -> u32 {
//         self.width
//     }

//     fn change_width(&mut self, new_width: u32) {
//         self.width = new_width;
//     }
// }

// struct MyString<'a> {
//     text: &'a str,
// }

fn main() {
    // let str1 = String::from("A string number 1");
    // let x = MyString{text: str1.as_str()};
    // let s: &'static str = "I have a static lifetime";

    // let user1 = User{active: true, username: String::from("Tony"), sign_in_count: 0};
    // println!("{} {} {}", user1.username, user1.active, user1.sign_in_count);
    // let user2 = build_user(String::from("Tony2"));
    // println!("{} {} {}", user2.username, user2.active, user2.sign_in_count);

    // let coords = Coordinates(1, 2, 3);
    // println!("{} {} {}", coords.0, coords.1, coords.2);

    // let mut square = Square {width: 5, height: 5};
    // println!("{}", square.area());
    // println!("{}", square.whats_my_width());
    // square.change_width(10);
    // println!("{}", square.whats_my_width());


    // let r;

    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("{}", r);
}

// fn example<'a>(x: &'a str) -> &'a str {
//     x
// }

// fn example1<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
//     x
// }

// fn build_user(username: String) -> User {
//     User {
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }