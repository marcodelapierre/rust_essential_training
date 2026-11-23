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
}
