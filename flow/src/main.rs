fn main() {
    // if conditionals

    let x = 3;
    let y = 5;

    if x > y {
        println!("x is greater than y");
    } else if x < y {
        println!("x is less than y");
    } else {
        println!("x is equal to y");
    }

    let z = if x == 3 {10} else {20};
    println!("z is {}", z);

    // loops

    let mut count = 0;
    let result = loop {
        if count == 10 {
            // break in loop can return a value (not in while)
            break count * 10;
        }
        count += 1;
        println!("count is {}", count);
    };
    println!("End of loop");
    println!("Final result is {}", result);

    count = 0;
    while count < 10 {
        count += 1;
        println!("count is {}", count);
    }

    let message = ['h', 'e', 'l', 'l', 'o' ];
//    for item in message.iter() {
    for item in message {
        println!("item is {}", item);
    }
    for (index, item) in message.iter().enumerate() {
        println!("item {} is {}", index, item);
    }
    for (index, &item) in message.iter().enumerate() {
        println!("item {} is {}", index, item);
        if item == 'e' {break};
    }

    for number in 0..5 {
        println!("number is {}", number);
    }

    let mut matrix = [[1,2,3],
                  [4,5,6],
                  [7,8,9]];
//    for row in matrix.iter() {
//        for num in row.iter() {
//    for row in matrix {
//        for num in row {
    for row in matrix.iter_mut() {
        for num in row.iter_mut() {
            // "*" added as advised by compiler
            *num += 10;
            print!("{}\t", num);
        }
        println!();
    }
}
