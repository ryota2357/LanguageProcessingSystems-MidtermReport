use crate::Expr;

use super::{OpCode, Stmt};

pub(super) fn compile(stmt: &Stmt) -> Vec<OpCode> {
    match stmt {
        Stmt::Assign(ident, expr) => {
            let mut opcodes = compile_expr(expr);
            opcodes.push(OpCode::SetLocal(ident.clone()));
            opcodes
        }
        Stmt::Expr(expr) => compile_expr(expr),
    }
}

fn compile_expr(expr: &Expr) -> Vec<OpCode> {
    match expr {
        Expr::Num(num) => vec![OpCode::LoadConst(*num)],
        Expr::Ident(ident) => vec![OpCode::LoadLocal(ident.clone())],
        Expr::Unary(op, expr) => {
            let mut opcodes = compile_expr(expr);
            match op {
                '+' => opcodes.push(OpCode::OpPos),
                '-' => opcodes.push(OpCode::OpNeg),
                '!' => opcodes.push(OpCode::OpFact),
                _ => unimplemented!("op: {}", op),
            }
            opcodes
        }
        Expr::Binary(op, lhs, rhs) => {
            let mut opcodes = compile_expr(lhs);
            opcodes.extend(compile_expr(rhs));
            match op {
                '+' => opcodes.push(OpCode::OpAdd),
                '-' => opcodes.push(OpCode::OpSub),
                '*' => opcodes.push(OpCode::OpMul),
                '/' => opcodes.push(OpCode::OpDiv),
                '^' => opcodes.push(OpCode::OpPow),
                _ => unimplemented!("op: {}", op),
            }
            opcodes
        }
        Expr::Call(ident, expr) => {
            let mut opcodes = compile_expr(expr);
            opcodes.push(OpCode::Call(ident.clone()));
            opcodes
        }
    }
}
