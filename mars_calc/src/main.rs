use std::io;

fn main() {
    println!("Enter your weight on Earth: ");
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let weight_on_earth: f32 = input.trim().parse().unwrap();
    dbg!(weight_on_earth);
    let weight_on_mars = calculate_weight_on_mars(weight_on_earth);

    // cannot change the value of a variable once it is set, unless it is mutable

    println!("Weight on Mars: {}kgen", weight_on_mars);
}

fn borrow_string(s: &String) {
    println!("{}", s);
}

fn own_string(s: String) {
    println!("{}", s);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
