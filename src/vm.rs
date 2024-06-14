use crate::OpCode;
use std::collections::HashMap;

pub(super) fn execute(opcodes: &[OpCode], env: &mut HashMap<String, f64>) -> Option<f64> {
    let mut stack = Vec::new();
    for opcode in opcodes {
        match opcode {
            OpCode::LoadConst(num) => stack.push(*num),
            OpCode::LoadLocal(ident) => {
                let Some(value) = env.get(ident) else {
                    panic!("Undefined variable: {}", ident);
                };
                stack.push(*value);
            }
            OpCode::SetLocal(ident) => {
                let value = stack.pop().unwrap();
                env.insert(ident.clone(), value);
            }
            OpCode::OpPos => {
                let value = stack.pop().unwrap();
                stack.push(value);
            }
            OpCode::OpNeg => {
                let value = stack.pop().unwrap();
                stack.push(-value);
            }
            OpCode::OpFact => {
                let value = stack.pop().unwrap();
                if (value.abs() - value) > 1e-8 || value < 0.0 {
                    panic!("Invalid factorial: {}", value);
                }
                let value = value.abs() as i64;
                let res = (1..=value).product::<i64>();
                stack.push(res as f64);
            }
            OpCode::OpAdd => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(lhs + rhs);
            }
            OpCode::OpSub => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(lhs - rhs);
            }
            OpCode::OpMul => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(lhs * rhs);
            }
            OpCode::OpDiv => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(lhs / rhs);
            }
            OpCode::OpPow => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(lhs.powf(rhs));
            }
            OpCode::Call(ident) => {
                let value = stack.pop().unwrap();
                match ident.as_str() {
                    "sqrt" => stack.push(value.sqrt()),
                    "abs" => stack.push(value.abs()),
                    _ => unimplemented!("function: {}", ident),
                }
            }
        }
    }
    stack.pop()
}
