use std::io;

fn main() {
    loop {
        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .expect("Failed to read line");

        if buf.trim().is_empty() {
            break;
        }

        let (mut a, mut b) = split_parse_input(&buf);
        a.reverse();
        b.reverse();
        println!("{}", calc_sum(a, b));
    }
}

fn split_parse_input(buf: &String) -> (Vec<char>, Vec<char>) {
    let mut binary_rep: Vec<Vec<char>> = Vec::new();

    for buf_slice in buf.trim().split_ascii_whitespace() {
        let n = match buf_slice.parse::<u32>() {
            Ok(n) => n,
            Err(_) => {
                println!("Failed to parse string");
                break;
            }
        };
        let b_string = format!("{:b}", n);
        let b_vector: Vec<char> = b_string.chars().collect();

        binary_rep.push(b_vector);
    }
    assert!(binary_rep.len() == 2);

    return (binary_rep[0].to_vec(), binary_rep[1].to_vec());
}

fn calc_sum(a: Vec<char>, b: Vec<char>) -> u32 {
    let (max_len, min_len) = if a.len() > b.len() {
        (a.len(), b.len())
    } else {
        (b.len(), a.len())
    };
    let mut sum: Vec<char> = vec!['0'; min_len];

    for i in 0..min_len {
        let xor = (a[i] as i32 ^ b[i] as i32) as u8;
        sum[i] = if xor == 1_u8 { '1' } else { '0' };
    }

    if min_len == b.len() {
        sum.append(&mut a[min_len..max_len].to_vec());
    } else {
        sum.append(&mut b[min_len..max_len].to_vec());
    }

    let mut total_sum: u32 = 0;
    for i in 0..max_len {
        let int_sum: u32 = sum[i] as u32 - '0' as u32;
        total_sum += int_sum * 2_u32.pow(i as u32);
    }

    return total_sum;
}
