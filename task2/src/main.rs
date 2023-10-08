use std::io;

// The simple operations for the calculator are initialized in this enum.
enum Operation {
    Add(f64,f64),
    Subtract(f64,f64),
    Multiply(f64,f64), 
    Divide(f64,f64),
}

fn main() {
    //Initialize the operator and the numbers
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut operation = String::new();

    //Enter the first number.
    println!("Enter the first number: ");
    io::stdin()
        .read_line(&mut num1)
        .expect("Could not read line.");
    
    //Enter the operation
    println!("Enter the operation: ");
    io::stdin()
        .read_line(&mut operation)
        .expect("Could not read line.");

    //Enter the second number
    println!("Enter the second number: ");
    io::stdin()
        .read_line(&mut num2)
        .expect("Could not read line.");

    //Parse the inputs
    let num1: f64 = num1.trim().parse().expect("Invalid number!");
    let operation: char = operation.trim().parse().expect("Invalid operation!");
    let num2: f64 = num2.trim().parse().expect("Invalid number!");

    //Define the operation 
    let result = match operation {
        '+' => calculate(Operation::Add(num1,num2)).to_string(),
        '-' => calculate(Operation::Subtract(num1,num2)).to_string(),
        '*' => calculate(Operation::Multiply(num1,num2)).to_string(),
        '/' => calculate(Operation::Divide(num1,num2)).to_string(),
        _ => "Invalid result".to_string(),
    };

    println!("The result is {}.", result);
}

//The function that does the calculation
fn calculate(oper: Operation) -> f64 {
    let result = match oper {
        Operation::Add(x,y) => x + y,
        Operation::Subtract(x,y) => x - y,
        Operation::Multiply(x,y) => x * y,
        Operation::Divide(x,y) => x / y,
    };

    result 
}
