fn main() {
    // integers
    let mut x: u16 = 255;
    println!("x is {}",x );
    x = x + 1;
    println!("x is {}",x );

    // floats
    let mut y = 10.0;
    println!("y is {}",y );
    y = y + 0.123456789123456789;
    println!("y is {}",y );
    let z: f32 = 10.123456789123456789;
    println!("z is {}",z );

    // arithmetic
    let a = 10;
    let b = 3;
    let c = a / b;
    println!("c is {}",c );
    let d = a as f64 / b as f64 + 1f64;
    println!("d is {}",d );
    println!("d is {0:8.3}, c is {1}, c is again {1}",d, c );
}
