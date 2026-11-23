use core::sync;

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

    // boolean
    let e:bool = true;
    let f:bool = false;
    println!("e is {}, f is {}",e, f );
    println!("not e is {}",!e );
    println!("e and f is {}", e & f );
    println!("e or f is {}", e | f );
    println!("e xor f is {}", e ^ f );
    // short-circuiting booleans
    println!("false AND PANIC is {}", false && panic!("PANIC!") );
    println!("true OR PANIC is {}", true || panic!("PANIC!") );
//    println!("true AND PANIC is {}", true && panic!("PANIC!") );

    // comparison
    let g = 10;
    let h = 20;
    println!("g is {}, h is {}",g, h );
    println!("g < h is {}", g < h );
    println!("g <= h is {}", g <= h );
    println!("g > h is {}", g > h );
    println!("g >= h is {}", g >= h );
    println!("g == h is {}", g == h );
    println!("g != h is {}", g != h );

    // characters
    let number = '1';
    let letter = 'a';
    let symbol = '$';
    let symbol2 = '\u{262D}'; // ☭
    let emoji = '😎';
    println!("number is {}, letter is {}, symbol is {}, symbol2 is {}, emoji is {}", number, letter, symbol, symbol2, emoji );
}
