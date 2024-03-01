use std::io;

pub fn calc() {
    println!("Simple Calculator");

    loop {
        println!("Enter an expression (e.g 2+2):");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();

        if input == "exit" {
            println!("Exiting calculator");
            break;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.len() != 3 {
            println!("Invalid expression. Please enter in the correct format");
            continue;
        }

        let num1: f64 = match parts[0].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number: {}", parts[0]);
                continue;
            }
        };

        let operator = parts[1];

        let num2: f64 = match parts[2].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number: {}", parts[2]);
                continue;
            }
        };

        let result = match operator {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 == 0.0 {
                    println!("Cannot divide by 0");
                    continue;
                } else {
                    num1 / num2
                }
            },
            _ => {
                println!("Invalid operator: {}", operator);
                continue;
            }
        };

        println!("Result: {}", result);
    }
}