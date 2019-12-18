struct User {
    username: String,
    email: String, 
    sign_in_count: u64,
    active: bool
}
fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("{}", user1.email);
    user1.email = String::from("anotheremail@example.com");
    println!("{}", user1.email);
    println!("{:?}", build_user(String::from("hi"), String::from("yo")).email);
    let user2 = User {
        username: String::from("hiello"),
        ..user1
    };
    println!("{}", user2.email);
    let t = (3,4,5);
    let index = 2;
    println!("{:?}", t);
    let (a, b) = (3, 5);
    let (a, b) = (b, a);
    println!("{}, {}", a, b);
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email, 
        username: username,
        active: true, 
        sign_in_count: 1,
    }
}