mod lib;
use lib::Tweet;
use lib::Summary;

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = &item;
        }
    }
    return largest;
}


fn largest_alt<T>(list: &[T]) -> T 
where T: PartialOrd + Copy {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    return largest;
}

use std::fmt::Display;

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn new(x: T, y: T) -> Self {
        Self {
            x, 
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("{:?}", self.x);
        } else {
            println!("{:?}", self.y);
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    println!("{:?}", largest(&number_list));
    println!("{:?}", &number_list);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    println!("{:?}", largest(&number_list));

    let char_list = vec!['y', 'm', 'a', 'q'];

    println!("{:?}", largest(&char_list));

    let integer = Point {x : 5, y : 10};
    let float = Point {x : 1.0, y : 4.0};
    println!("{:?}, {:?}", integer, float);
    println!("{:?}", integer.x);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false
    };

    println!("1 new tweet: {:?}", tweet.summarize());
}
