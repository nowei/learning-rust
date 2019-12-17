fn another_function(x: i32, y: i32) {
    println!("x = {}", x);
    println!("y = {}", y);
}
fn five() -> i32 {
    5
}
fn returned_five() -> i32 {
    return five();
}
fn main() {
    println!("Hello, world!");
    another_function(5, 3);
    println!("five() = {}", five());
    println!("returned_five() = {}", returned_five());
    // Scope testing
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("{}, {}", x, y);
}
