mod Calculator;
use std::env;

fn main() {
    let calculator = Calculator::Calculator::new();
    let args: Vec<String> = env::args().collect();

    let operand1 = args[1].parse::<f32>().unwrap();
    let operand2 = args[3].parse::<f32>().unwrap();
    let operator = &(*args[2]);

    match operator{
        "*" => {
            println!("{}", calculator.multiply(&operand1, &operand2));
        }
        "/" => {
            println!("{}", calculator.divide(&operand1, &operand2));
        }
    }
}
