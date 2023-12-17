#![allow(dead_code)]

use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
};

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Num(f64),
    Op(char),
    Ident(String),
}

#[derive(Debug, Clone, PartialEq)]
enum Expr {
    Num(f64),
    Ident(String),
    Unary(char, Box<Expr>),
    Binary(char, Box<Expr>, Box<Expr>),
    Call(String, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
enum Stmt {
    Assign(String, Expr),
    Expr(Expr),
}

fn eval(stmt: &Stmt, map: &mut HashMap<String, f64>) -> (Option<String>, f64) {
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

fn main() {
    fn question(msg: &str) -> String {
        let mut stdout = stdout();
        stdout.write_all(msg.as_bytes()).unwrap();
        stdout.flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input
    }

    let mut map = HashMap::new();
    loop {
        let line = question("> ");
        let tokens = lexer::parse(&line);
        let stmt = parser::parse(&tokens);
        let res = eval(&stmt, &mut map);
        match res {
            (Some(name), res) => println!("{} = {}", name, res),
            (None, res) => println!("= {}", res),
        }
    }
}

mod lexer {
    use super::Token;

    pub fn parse(input: &str) -> Vec<Token> {
        let mut chars = input.chars().peekable();
        let mut tokens = Vec::new();
        while let Some(c) = chars.next() {
            if c.is_ascii_whitespace() {
                continue;
            }
            match c {
                '0'..='9' => {
                    let mut num = c.to_string();
                    loop {
                        match chars.peek() {
                            Some(c) if c.is_ascii_digit() || *c == '.' => {
                                num.push(*c);
                                chars.next();
                            }
                            _ => break,
                        }
                    }
                    tokens.push(Token::Num(num.parse().unwrap()))
                }
                '+' | '-' | '*' | '/' | '^' | '!' | '(' | ')' | '=' => tokens.push(Token::Op(c)),
                'a'..='z' | 'A'..='Z' => {
                    let mut name = c.to_string();
                    loop {
                        match chars.peek() {
                            Some(c) if c.is_ascii_alphanumeric() => {
                                name.push(*c);
                                chars.next();
                            }
                            _ => break,
                        }
                    }
                    tokens.push(Token::Ident(name));
                }
                _ => panic!("Unexpected char: {:?}", c),
            }
        }
        tokens
    }
}
