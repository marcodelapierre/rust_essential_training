use std::env;
use std::fs;
use std::io::prelude::*;

fn main() {
    // CLI
    if env::args().len() <= 2 {
        println!("Program requires at least 2 arguments.");
        return;
    }

    for (index, argument) in env::args().enumerate() {
        println!("Argument {} is {}", index, argument);
    }

    let arg2 = env::args().nth(2).unwrap();
    println!("PRINT: Argument 2 is {}", arg2);

    // Files - read
    let contents = fs::read_to_string("planets.txt").unwrap();
    println!("contents is:\n{}", contents);

    for line in contents.lines() {
        println!("line is {}", line);
    }

    let contents_byte = fs::read("planets.txt").unwrap();
    println!("contents is:\n{:?}", contents_byte);

    // Files - write
    let mut speech = String::new();
    speech.push_str("We choose to go to the Moon in this decade\n");
    speech.push_str("and do the other things,\n");
    speech.push_str("not because they are easy,\n");
    speech.push_str("but because they are hard.");

    fs::write("speech.txt", speech);

    // Files - append
    fs::write("planets_new.txt", contents);
    let mut file = fs::OpenOptions::new().append(true).open("planets_new.txt").unwrap();
    file.write(b"\nPlutone");
}
