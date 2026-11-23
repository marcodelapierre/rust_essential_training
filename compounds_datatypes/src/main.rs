fn main() {
    let letters = ['a', 'b', 'c', 'd', 'e'];
    let first_letter = letters[0];
    println!("The first letter is: {}", first_letter);

    let numbers: [i32; 5];
    //println!("The last number is: {}", numbers[4]);
    numbers = [1, 2, 3, 4, 5];
    println!("The last number is: {}", numbers[4]);

    let numbers2 = [0; 5];
    let index: usize = numbers2.len(); // use usize as type for index
    println!("The last number is: {}", numbers2[index-1]);

    // multi-dimensional arrays
    let parking_lot = [[1, 2, 3], 
                                      [4, 5, 6]];
    let array_value = parking_lot[0][1];
    println!("The value is: {}", array_value);

    let _garage = [[[0; 100]; 20]; 5]; // compiler suggests using underscores for unused variables

    // tuple
    let stuff = (5, 3.14, 'x');
    println!("The first value is: {}", stuff.0);

    let (x, y, z) = stuff;
    println!("The second value is: {}", y);
}
