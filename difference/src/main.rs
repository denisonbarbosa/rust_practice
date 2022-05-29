use std::io;

fn main() {
    let mut values: Vec<i32> = Vec::new();
    collect_input(&mut values);

    let diff = calc_difference(&values);
    println!("DIFERENCA = {}", &diff);
}

fn collect_input(values: &mut Vec<i32>) {
    for _ in 0..4 {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Unable to read line");
        let value: i32 = match input.trim().parse() {
            Ok(i) => i,
            Err(_) => {
                println!("Failed to parse input");
                break;
            }
        };
        values.push(value);
    }
}

fn calc_difference(values: &Vec<i32>) -> i32 {
    let a_x_b = values[0] * values[1];
    let c_x_d = values[2] * values[3];

    return a_x_b - c_x_d;
}
