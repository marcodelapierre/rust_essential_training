fn main() {
    let test1 = "We need more space.";
    assert_eq!(trim_spaces(test1), "We need more space.");
    
    let test2 = String::from("   There's space in front.");
    assert_eq!(trim_spaces(&test2), "There's space in front.");
    
    let test3 = String::from("There's space to the rear. ");
    assert_eq!(trim_spaces(&test3[..]), "There's space to the rear.");   
    
    let test4 = "  We're surrounded by space!    ";
    assert_eq!(trim_spaces(test4), "We're surrounded by space!");
    
    let test5 = "     ";
    assert_eq!(trim_spaces(test5), "");
    
    let test6 = "";
    assert_eq!(trim_spaces(test6), "");
    
    let test7 = " 🚀 ";
    assert_eq!(trim_spaces(test7), "🚀");
    println!("Tests passed!");
}

fn trim_spaces(s: &str) -> &str {
// .as_bytes() converts to byte slice (requires .iter() and "&item" enumeration)
// .bytes() returns a byte enumerator (requires "item" enumeration)
    //    let bytes = s.as_bytes();
    let mut first: usize = 0;
    let mut last: usize = 0;

    // NOTE that the current implementation blends char and byte length
    // - enumeration is at the char level
    // - &str length is in bytes
    // NOTE this only works because 
    // - space characters are 1 byte per 1 char
    // - focus is on lead and trail, we don't count through special chars

    // for (index, &item) in bytes.iter().enumerate(){
    //     if item != b' ' {
    for (index, item) in s.chars().enumerate(){
        if item != ' ' {
            first = index;
            break;
        }
    }

    // for (index, &item) in bytes.iter().rev().enumerate(){
    //     if item != b' ' {
    //         last = s.as_bytes().len() - index;
    for (index, item) in s.chars().rev().enumerate(){
        if item != ' ' {
            last = s.len() - index;
            break;
        }
    }
    return &s[first..last];
}