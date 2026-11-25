fn main() {
    let planet = "Earth";
    {
        println!("planet is {}", planet);
        // shadowing (inner)
        // note that the 2nd variable points to a NEW memory location
        // old memory location is unaffected
        let mut planet = 4;
        println!("planet is {}", planet);
    }
    println!("planet is {}", planet);

    // shadowing (outer)
    let planet = "Mars";
    println!("planet is {}", planet);

    let mut message = String::from("Earth");
    println!("message is {}", message);
    message.push_str(" is home.");
    println!("message is {}", message);

    // ownership
    // heap variables
    let outer_planet: String;
    {
        let inner_planet = String::from("Mercury");
        println!("inner_planet is {}", inner_planet);
        outer_planet = inner_planet.clone();
//        outer_planet = inner_planet; // this results in an error
        println!("inner_planet is {}", inner_planet);
    }
    println!("outer_planet is {}", outer_planet);
    // stack variables are ok
    let zio1 = 5;
    let zio2 = zio1;
    println!("zio1 is {}", zio1);
    // heap ownership and functions
    let rocket_fuel = String::from("RP-1");
    // makes sense to shadow: we don't want to make the variable mutable
    // also, ownership of the original value was transfered via the function
    let mut rocket_fuel = process_fuel(rocket_fuel);
    println!("rocket_fuel is {}", rocket_fuel);

    // borrowing (references)
    process_fuel_2(&rocket_fuel);
    println!("rocket_fuel is {}", rocket_fuel);

    // only 1 mutable ref per variable per scope
    // to avoid risk of data races
    process_fuel_3(&mut rocket_fuel);
    println!("rocket_fuel is {}", rocket_fuel);

}

fn process_fuel(propellant: String) -> String {
    println!("processing propellant {}...", propellant);
    return propellant;
}

fn process_fuel_2(propellant: &String) {
    println!("processing propellant {}...", propellant);
}
fn process_fuel_3(propellant: &mut String) {
    println!("processing propellant {}...", propellant);
    propellant.push_str(" is highly flammable!");
}