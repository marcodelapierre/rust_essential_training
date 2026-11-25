fn main() {
    say_hello();
    say_hello();
    let x = 1;
    let y = 2;
    say_sum(x, y);
    say_a_number(x as i32);
    let result = square2 (13);
    println!("The result is {}", result.1);
    println!("The tuple is {:?}", result);
}

fn say_hello(){
    println!("Hello!");
    say_a_number(13);
}

fn say_a_number(number: i32){
    println!("The number is: {}", number);
}

fn say_sum(a: u8, b: u8){
    println!("The sum is: {}", a + b);
}

fn _square(x: i32) -> i32 {
    println!("Squaring {}", x);
    x * x
}

fn square2(x: i32) -> (i32, i32) {
    println!("Squaring {}", x);
    (x, x * x)
}

fn _square3(x: i32) -> (i32, i32) {
    println!("Squaring {}", x);
    return (x, x * x);
    println!("This is after the return");
}
