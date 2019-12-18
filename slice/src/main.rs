fn main() {
    let s = String::from("henlo world");
    println!("length of first word of {:?} is {}!", s, first_word(&s).len());
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{:?} {:?}", hello, world);
    let slice = &s[0..2];
    let slice = &s[..2];
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("{:?}, {:?}", a, slice);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}