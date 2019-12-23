use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::error::Error;

use std::io;
use std::io::Read;

fn read_username_from_file_cleanest() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn read_username_from_file_clean() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    return Ok(s);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = match File::open("hello.txt") {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

fn testing() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;
    Ok(())
}

fn main() {
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];
    // v[99];

    let f = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            }, 
            other_error => panic!("Problem opening the file {:?}", other_error),
        }
    };
    println!("{:?}", f);

    let g = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    println!("{:?}", g);

    // let f = File::open("henlo.txt").unwrap();
    // let f = File::open("henlo.txt").expect("Failed to open hello.txt");
    let s = read_username_from_file();
    println!("{:?}", s);
    println!("{:?}", read_username_from_file_clean());
    println!("{:?}", read_username_from_file_cleanest());
    println!("{:?}", testing());
}