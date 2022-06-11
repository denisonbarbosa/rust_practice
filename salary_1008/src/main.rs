use std::io;

#[derive(Debug)]
struct Employee {
    number: i32,
    worked_hours: i32,
    amount_per_hour: f32,
    salary: f32
}

impl Employee {
    fn calc_salary(&mut self) {
        self.salary = (self.worked_hours as f32) * self.amount_per_hour;
    }
}

fn main() {
    let emp: Employee = collect_parse_input();
    println!("NUMBER = {}\nSALARY = U$ {:.2}", emp.number, emp.salary);

}

fn collect_parse_input() -> Employee {
    let emp_number_string: String = read_stdin_line();

    let emp_number: i32 = match emp_number_string.parse() {
        Ok(i) => i,
        Err(err) => {
            println!("{}", err);
            -1
        }
    };

    let worked_hours_string: String = read_stdin_line();
    let worked_hours: i32 = match worked_hours_string.parse() {
        Ok(i) => i,
        Err(err) => {
            println!("{}", err);
            -1
        }
    };

    let amount_per_hour_string: String = read_stdin_line();
    let amount_per_hour: f32 = match amount_per_hour_string.parse() {
        Ok(f) => f,
        Err(err) => {
            println!("{}", err);
            -1.0
        }
    };

    let mut employee = Employee {
        number: emp_number,
        worked_hours: worked_hours,
        amount_per_hour: amount_per_hour,
        salary: 0.0
    };
    employee.calc_salary();
    return employee;
}

fn read_stdin_line() -> String {
    let mut buf = String::new();

    io::stdin().read_line(&mut buf).expect("Error reading line");

    return buf.trim().to_string();
}
