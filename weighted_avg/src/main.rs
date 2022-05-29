use std::io;

fn main() {
    let mut grades: Vec<f32> = Vec::new();

    collect_input(&mut grades);

    let avg = calc_avg(&grades);

    println!("MEDIA = {:.1}", &avg);
}

fn calc_avg(grades: &Vec<f32>) -> f32 {
    let mut average: f32 = 0.0;
    for (i, &grade) in grades.iter().enumerate() {
        average += match i {
            0 => grade * 0.2,
            1 => grade * 0.3,
            2 => grade * 0.5,
            _ => 0.0,
        };
    }
    return average;
}

fn collect_input(grades: &mut Vec<f32>) {
    for _ in 0..3 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");

        let grade: f32 = match input.trim().parse::<f32>() {
            Ok(n) => n,
            Err(_) => {
                println!("Did you correctly input a float?");
                0.0
            }
        };

        grades.push(grade);
    }
}
