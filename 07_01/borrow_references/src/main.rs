fn main() {
    let rocket_fuel = String::from("RP-1");
    // let (rocket_fuel, length) = process_fuel(rocket_fuel);
    let length = process_fuel(&rocket_fuel);
    println!("rocket_fuel length is {}", length);
}

fn process_fuel(propellant: &String) -> usize {
    println!("processing propellant {}...", propellant);
    let length = propellant.len();
    // (propellant, length)
    length
}
