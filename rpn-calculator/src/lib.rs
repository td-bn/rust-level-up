#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn calc(f: i32, s: i32, op: &CalculatorInput) -> i32 {
    match op {
        CalculatorInput::Add => s + f,
        CalculatorInput::Subtract => s - f,
        CalculatorInput::Multiply => s * f,
        CalculatorInput::Divide => s / f,
        _ => 0,
    }
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();
    for i in inputs {
        match i {
            CalculatorInput::Multiply | CalculatorInput::Divide | 
                CalculatorInput::Subtract | CalculatorInput::Add => {
                if stack.len() < 2 {
                    return None;
                }
                let (x, y) = (stack.pop().unwrap(), stack.pop().unwrap());
                stack.push(calc(x, y, i));                
            }
            CalculatorInput::Value(x) => {
                stack.push(*x);
            }
        }
    }
    match stack.len() {
        1 => stack.pop(),
        _ => None
    }
}
