fn try_vectors() {
    let mut v: Vec<i32> = Vec::new();
    let vp = vec![1,2,3];
    println!("{:?}, {:?}", v, vp);
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}", v);
    let mut temp = ret_vec();
    temp.push(6);
    println!("{:?}", temp);

    // This is actually really annoying
    let third: &i32 = &v[2];
    println!("The third element is {:?}", third);
    
    matching(&v, 9);
    matching(&v, 3);

    for i in &v {
        print!("{:?} ", i);
    }
    println!();

    for i in &mut v {
        *i += 55;
    }
    println!("{:?}", v);

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("hielo")),
        SpreadsheetCell::Float(41234.32),
    ];
    println!("{:?}", row);
}

fn ret_vec() -> Vec<i32> {
    return Vec::new();
}

fn matching(v: &Vec<i32>, index: usize) {
    match v.get(index) {
        Some(value) => println!("The {}-th element is {:?}", index, value),
        None => println!("There is no element"),
    }
}

fn try_strings() {
    let mut s = String::new();
    let data = "Initial contents"; 
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");
    // Apparently strings support other langauges. 
    let hello = String::from("שָׁלוֹם");
    let mut s = String::from("foo");
    s = s + "yello";
    println!("{:?}", &s);
    s.push_str("bar");
    println!("{:?}", &s);
    s.push('b');
    println!("{:?}", &s);
    let other = s + &String::from("hello");
    println!("{:?}", &other);
    
    // Format doesn't take ownership
    let fin = format!("{}, {}, {} or something", other, "what", "wow");
    println!("{:?}", other);
    println!("{:?}", fin);

    // indexing into strings
    let s1 = String::from("Здравствуйте");
    let h = &s1[0..4];
    println!("{:?}", h);

    for c in "नमस्ते".chars() {
        println!("{:?}", c);
    }
    for b in "नमस्ते".bytes() {
        println!("{:?}", b);
    }
}

use std::collections::HashMap;

fn try_maps() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("{:?}", map);

    let mut scores = HashMap::new();
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{:?}", score);

    for (key, value) in &scores {
        println!("{:?}: {:?}", key, value);
    }
    scores.insert(String::from("Blue"), 25);
    scores.insert(String::from("Blue"), 30);
    println!("{:?}", scores.get(&String::from("Blue")));

    let mut temp = scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}

fn main() {
    try_vectors();
    try_strings();
    try_maps();
}
