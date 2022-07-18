use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let earth_weight: f32 = match input.trim().parse() {
        Ok(weight) => weight,
        Err(err) => {
            println!("{}: {}", input, err);
            0.0
        }
    };

    let mars_weight = calculate_weight_on_mars(earth_weight);
    println!("Weight on mars = {}", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    return weight / 9.81 * 3.711;
}
