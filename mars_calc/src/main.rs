use std::io;

fn main() {

    let mut input = String::new();
    
    let mut s1 = &mut input;
    let s2 = &input;

    println!("{} {}", s1, s2);

    some_fn(&mut input);

    io::stdin().read_line(&mut input);

    // typed variables
    let weight_on_earth = 72.0;
    let weight_on_mars = calculate_weight_on_mars(weight_on_earth);

    // cannot change the value of a variable once it is set, unless it is mutable
    // weight_on_mars = 72.0;

    println!("Weight on Mars: {}kgen", weight_on_mars);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}


fn some_fn(s: &mut String) {
    s.push_str("hello");
}