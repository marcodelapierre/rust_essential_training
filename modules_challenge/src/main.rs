use std::io;
use rand;

fn main() {
    let mut guess_str = String::new();
    let mut guess: i32 = 0;

    let secret: i32 = rand::random_range(1..101);

    while guess != secret {
        println!("Guess the number");
        let _ = io::stdin().read_line(&mut guess_str);
        println!("pippo1 {}", guess_str);
        guess = guess_str.trim().parse().unwrap();
        println!("pippo2 {}", guess);

        if guess < secret {
            println!("Your guess was less than the number: {}", guess)
        } else if guess > secret {
            println!("Your guess was greater than the number: {}", guess)
        }
        guess_str.clear();
    }

    println!("You WON! Number was {}", secret);

}
