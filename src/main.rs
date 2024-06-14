use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
};

mod interpreter;
mod lexer;
mod parser;

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

fn main() {
    let mode = loop {
        let res = question("Mode (1: interpreter, 2: vm): ")
            .trim()
            .parse::<u32>();
        match res {
            Ok(1) => break 1,
            Ok(2) => break 2,
            _ => continue,
        }
    };

    let mut env = HashMap::new();
    if mode == 1 {
        interpreter_mode(&mut env);
    } else {
        vm_mode(&mut env);
    }
}

fn question(msg: &str) -> String {
    let mut stdout = stdout();
    stdout.write_all(msg.as_bytes()).unwrap();
    stdout.flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
}

fn interpreter_mode(env: &mut HashMap<String, f64>) {
    loop {
        let line = question("> ");
        let tokens = lexer::parse(&line);
        let stmt = parser::parse(&tokens);
        let res = interpreter::eval(&stmt, env);
        match res {
            (Some(name), res) => println!("{} = {}", name, res),
            (None, res) => println!("= {}", res),
        }
    }
}

fn vm_mode(_env: &mut HashMap<String, f64>) {
    unimplemented!();
}
