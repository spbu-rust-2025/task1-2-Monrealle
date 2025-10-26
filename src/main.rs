use std::io;

fn main() {
    let mut sum = 0;
    let mut invalid_input = false; // Было: ivalid_input (опечатка)

    loop {
        let mut input = String::new(); // Было: lt mup input (опечатки)

        if io::stdin().read_line(&mut input).is_err() {
            invalid_input = true;
            continue;
        }

        let input = input.trim();

        if input == "-1" {
            break;
        }

        match input.parse::<i64>() {
            Ok(num) => {
                if num > 0 {
                    sum += num;
                } else {
                    invalid_input = true;
                }
            }
            Err(_) => {
                invalid_input = true;
            }
        }
    }

    if invalid_input {
        println!("NaN");
    } else {
        println!("{}", sum);
    }
}
