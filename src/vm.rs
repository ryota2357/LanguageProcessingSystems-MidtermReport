use crate::Instr;
use std::collections::HashMap;

pub(super) fn execute(instrs: &[Instr], env: &mut HashMap<String, f64>) -> Option<f64> {
    let mut stack = Vec::new();
    for instr in instrs {
        match instr {
            Instr::LoadConst(num) => stack.push(*num),
            Instr::LoadLocal(ident) => {
                let Some(value) = env.get(ident) else {
                    panic!("Undefined variable: {}", ident);
                };
                stack.push(*value);
            }
            Instr::SetLocal(ident) => {
                let value = stack.pop().unwrap();
                env.insert(ident.clone(), value);
            }
            Instr::OpPos => {
                let value = stack.pop().unwrap();
                stack.push(value);
            }
            Instr::OpNeg => {
                let value = stack.pop().unwrap();
                stack.push(-value);
            }
            Instr::OpFact => {
                let value = stack.pop().unwrap();
                if (value.abs() - value) > 1e-8 || value < 0.0 {
                    panic!("Invalid factorial: {}", value);
                }
                let value = value.abs() as i64;
                let res = (1..=value).product::<i64>();
                stack.push(res as f64);
            }
            Instr::OpAdd => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(lhs + rhs);
            }
            Instr::OpSub => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(lhs - rhs);
            }
            Instr::OpMul => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(lhs * rhs);
            }
            Instr::OpDiv => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(lhs / rhs);
            }
            Instr::OpPow => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(lhs.powf(rhs));
            }
            Instr::Call(ident) => {
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
