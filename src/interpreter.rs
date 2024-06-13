use super::{Expr, Stmt};
use std::collections::HashMap;

pub(super) fn eval(stmt: &Stmt, map: &mut HashMap<String, f64>) -> (Option<String>, f64) {
    fn _eval(expr: &Expr, map: &HashMap<String, f64>) -> f64 {
        match expr {
            Expr::Num(x) => *x,
            Expr::Ident(name) => match map.get(name) {
                Some(x) => *x,
                None => panic!("Unknown variable: {}", name),
            },
            Expr::Unary(op, expr) => match op {
                '+' => _eval(expr, map),
                '-' => -_eval(expr, map),
                '!' => {
                    let expr = _eval(expr, map);
                    if (expr.abs() - expr) > 1e-8 || expr < 0.0 {
                        panic!("Invalid factorial: {}", expr);
                    }
                    let expr = expr.abs() as i64;
                    let res = (1..=expr).product::<i64>();
                    res as f64
                }
                _ => unimplemented!("op: '{}'", op),
            },
            Expr::Binary(op, lhs, rhs) => match op {
                '+' => _eval(lhs, map) + _eval(rhs, map),
                '-' => _eval(lhs, map) - _eval(rhs, map),
                '*' => _eval(lhs, map) * _eval(rhs, map),
                '/' => _eval(lhs, map) / _eval(rhs, map),
                '^' => {
                    let lhs = _eval(lhs, map);
                    let rhs = _eval(rhs, map);
                    if rhs < 0.0 {
                        panic!("Invalid exponent: {}", rhs);
                    }
                    lhs.powf(rhs)
                }
                _ => unimplemented!("op: '{}'", op),
            },
            Expr::Call(name, arg) => match name.as_str() {
                "sqrt" => _eval(arg, map).sqrt(),
                "abs" => _eval(arg, map).abs(),
                _ => unimplemented!("name: '{}'", name),
            },
        }
    }
    match stmt {
        Stmt::Assign(name, expr) => {
            let res = _eval(expr, map);
            map.insert(name.clone(), res);
            (Some(name.clone()), res)
        }
        Stmt::Expr(expr) => (None, _eval(expr, map)),
    }
}
