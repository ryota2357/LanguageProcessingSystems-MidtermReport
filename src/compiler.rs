use crate::Expr;

use super::{Instr, Stmt};

pub(super) fn compile(stmt: &Stmt) -> Vec<Instr> {
    match stmt {
        Stmt::Assign(ident, expr) => {
            let mut instrs = compile_expr(expr);
            instrs.push(Instr::SetLocal(ident.clone()));
            instrs
        }
        Stmt::Expr(expr) => compile_expr(expr),
    }
}

fn compile_expr(expr: &Expr) -> Vec<Instr> {
    match expr {
        Expr::Num(num) => vec![Instr::LoadConst(*num)],
        Expr::Ident(ident) => vec![Instr::LoadLocal(ident.clone())],
        Expr::Unary(op, expr) => {
            let mut instrs = compile_expr(expr);
            match op {
                '+' => instrs.push(Instr::OpPos),
                '-' => instrs.push(Instr::OpNeg),
                '!' => instrs.push(Instr::OpFact),
                _ => unimplemented!("op: {}", op),
            }
            instrs
        }
        Expr::Binary(op, lhs, rhs) => {
            let mut instrs = compile_expr(lhs);
            instrs.extend(compile_expr(rhs));
            match op {
                '+' => instrs.push(Instr::OpAdd),
                '-' => instrs.push(Instr::OpSub),
                '*' => instrs.push(Instr::OpMul),
                '/' => instrs.push(Instr::OpDiv),
                '^' => instrs.push(Instr::OpPow),
                _ => unimplemented!("op: {}", op),
            }
            instrs
        }
        Expr::Call(ident, expr) => {
            let mut instrs = compile_expr(expr);
            instrs.push(Instr::Call(ident.clone()));
            instrs
        }
    }
}
