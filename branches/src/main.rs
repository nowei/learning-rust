fn main() {
    let number = -3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    let y = {
        let x = 1;
        1 + x
    };
    let x = if y > 3 { 3 } else { 5 };
    println!("x = {}", x);
}
