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

    println!("Enter the first number: ");
    io::stdin()
        .read_line(&mut num1)
        .expect("Could not read line.");

}

//The function that does the calculation
fn calculate(oper: Operation) -> f64 {
    match oper {
        Operation::Add(x,y) => x + y,
        Operation::Subtract(x,y) => x - y,
        Operation::Multiply(x,y) => x * y,
        Operation::Divide(x,y) => x / y,
    }
}
