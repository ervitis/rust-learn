struct Global {
    name: String,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    // for empty structs
    #[derive(Debug, Default)]
    struct User {
        username: String,
        email: String,
        active: bool,
    }

    let global = Global {
        name: String::from("example"),
    };

    println!("global = {}", global.name);

    let mut user = User::default();
    user.username = String::from("victor");

    println!("user {} and email {}", user.username, user.email);

    let user2 = User {
        email: String::from("myemail@example.com"),
        active: true,
        username: String::from("newUser"),
    };
    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("thisworks"),
        ..user2
    };

    println!("User3 {}<{}> and user2 {}<{}>", user3.username, user3.email, user2.username, user2.email);
    println!("User3 {}<{}> and user2 {}<{}>", user3.username, user3.active, user2.username, user2.active);

    let rectangle = Rectangle {width: 30, height: 40};
    // println!("Area={}", area(rectangle));

    println!("Area={:?}", rectangle.area());

    let rect2 = Rectangle {width: 20, height: 10};
    println!("Can hold {}", rectangle.can_hold(&rect2));
}

fn area(rectangle: Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
