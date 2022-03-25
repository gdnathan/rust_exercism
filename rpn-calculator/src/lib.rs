#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();

    for input in inputs {
        match input {
            CalculatorInput::Value(val) => stack.push(*val),
            _ => {
                let right = stack.pop()?;
                let left = stack.pop()?;
                match input {
                    CalculatorInput::Add => stack.push(left + right),
                    CalculatorInput::Subtract => stack.push(left - right),
                    CalculatorInput::Multiply => stack.push(left * right),
                    CalculatorInput::Divide => stack.push(left / right),
                    _ => unreachable!(),
                }
            }
        }
    }
    if stack.len() != 1 {
        None
    } else {
        stack.pop()
    }
}
