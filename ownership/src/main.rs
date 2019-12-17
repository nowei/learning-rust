fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{:?}", s);

    let x = 5;
    let y = x;
    println!("{:?}, {:?}", x, y);

    let s1 = String::from("hello");
    let s2 = s1.clone(); // Note, using &s1 gives pointer copy
    println!("{:?}, {:?}", s1, s2);

    let s = String::from("hello");
    takes_ownership(s);
    let x = 5;
    makes_copy(x);

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);

    let s1 = String::from("hello");
    println!("{:?} outside function", s1);
    pass_in_string(&s1);
    println!("{:?} outside function", s1);
}

fn takes_ownership(some_string: String) {
    println!("{:?}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{:?}", some_integer);
}

fn gives_ownership() -> String {
    return String::from("hello");
}

fn takes_and_gives_back(some_string: String) -> String {
    return some_string;
}

fn calculate_length(some_string: String) -> (String, usize) {
    let length = some_string.len();
    return (some_string, length);
}

fn pass_in_string(some_string: &String) {
    println!("{:?} inside function", some_string);
}
