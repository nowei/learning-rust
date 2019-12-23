use std::fs::File;
fn main() {
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];
    // v[99];

    let f = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error);
        }
    };
    println!("{:?}", f);
}
