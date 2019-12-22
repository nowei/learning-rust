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

    

}


fn main() {
    try_vectors();
    try_strings();
}
