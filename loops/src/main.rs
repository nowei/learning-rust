fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2
        }
    };
    println!("{:?}, {}", counter, result);

    let mut number = 3;
    while number != 0 {
        println!("{:?}!", number);
        number -= 1;
    }
    println!("{:?}", number);

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < a.len() {
        println!("a[index] = {}", a[index]);
        index += 1;
    }

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("{:?}", element);
    }
}
