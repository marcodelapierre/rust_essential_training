use std::env;
use std::fs;

fn main() {
    if env::args().len() <= 2 {
        // eprint prints to stderr
        eprintln!("Program requires at least 2 arguments.");
        //return;
        std::process::exit(1);
    }

    let record_file = env::args().nth(1).unwrap();
    let astro_name  = env::args().nth(2).unwrap();

    let contents = fs::read_to_string(record_file).unwrap();

    for line in contents.lines() {
        if line == astro_name {
            println!("Astronaut found. GO!");
            return;
        }
    }
    println!("Astronaut NOT found.");
}
