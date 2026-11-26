fn main() {
    let message = String::from("Greetings from Earth!");
    println!("message is {}", message);

    // NOTE: string length is in bytes, not chars
    // careful when string has special characters
    let last_word = &message[15..15+5];
    //let last_word = &message[15..];
    println!("last_word is {}", last_word);

    let planets = [1, 2, 3, 4, 5, 6, 7, 8];
    let inner_planets: &[i32] = &planets[..4];
    // format {:?} required by array
    println!("inner_planets are {:?}", inner_planets);
}
