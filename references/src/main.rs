fn main() {
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of {:?} is {:?}", s1, len);
    change(&mut s1);
    println!("{:?}", s1);
    {
        let r1 = &mut s1;
    }
    let r2 = &mut s1;
    println!("{}", r2);

    let r1 = &s1;
    let r3 = &s1;
    println!("{}, {}", r1, r3);
}

fn calculate_length(s: &String) -> usize {
    return s.len();
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}