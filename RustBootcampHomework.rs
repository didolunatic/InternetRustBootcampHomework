use std::io;

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => a / b,
    }
}

fn main() {
    println!("Enter the first number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let first_number: f64 = input.trim().parse().expect("Invalid number");

    println!("Enter the operation (+, -, *, /):");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let operation: char = input.trim().chars().next().expect("Invalid input");

    println!("Enter the second number:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let second_number: f64 = input.trim().parse().expect("Invalid number");

    let operation_enum = match operation {
        '+' => Operation::Add(first_number, second_number),
        '-' => Operation::Subtract(first_number, second_number),
        '*' => Operation::Multiply(first_number, second_number),
        '/' => Operation::Divide(first_number, second_number),
        _ => {
            println!("Invalid operation.");
            return;
        }
    };

    let result = calculate(operation_enum);
    println!("Result: {}", result);
}
