
use std::io;

fn main() {
    loop {
        println!("Enter an expression (e.g. 2 + 2 | 2 - 2  | e.g. 2 / 2 | 2 * 2):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();

        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.len() != 3 {
            println!("Invalid expression. Please try again.");
            continue;
        }

        let num1: f32 = match parts[0].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number. Please try again.");
                continue;
            }
        };

        let operator = parts[1];

        let num2: f32 = match parts[2].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number. Please try again.");
                continue;
            }
        };

        let result = match operator {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 == 0.0 {
                    println!("Division by zero is not allowed. Please try again.");
                    continue;
                }
                num1 / num2
            },
            _ => {
                println!("Invalid operator. Please try again.");
                continue;
            }
        };

        println!("Result: {}", result);

        println!("Do you want to perform another calculation? (yes/no)");

        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("Failed to read line");

        if answer.trim().to_lowercase() != "yes" {
            break;
        }
    }
}


