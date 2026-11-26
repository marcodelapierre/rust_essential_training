use std::io;
use rand;

fn main() {
    let mut buffer = String::new();
    println!("Enter a message:");
    let _ = io::stdin().read_line(&mut buffer);
    println!("buffer is {}", buffer);

    let number: i32 = buffer.trim().parse().unwrap(); //.expect("REASON");
    //let number = buffer.trim().parse::<i32>().unwrap();
    println!("number + 1 is {}", number + 1);

    let number2 = rand::random::<f64>();
    println!("number2 is {}", number2);
}
