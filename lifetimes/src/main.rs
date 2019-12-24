fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        return s1;
    } else {
        return s2;
    }
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where T: Display {
    println!("{}", ann);
    if x.len() > y.len() {
        return x;
    } else {
        return y;
    }
}

fn main() {
    let string1 = String::from("abcd");
    {
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
                              .next()
                              .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
    println!("{:?}", i);
    let string2 = "xyz";
    longest_with_an_announcement(string1.as_str(), string2, "hello");
}
