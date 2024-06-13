use super::{Expr, Stmt, Token};
use std::{iter::Peekable, slice::Iter};

pub fn parse(tokens: &[Token]) -> Stmt {
    match (tokens.first(), tokens.get(1)) {
        (None, _) => panic!("Unexpected EOI"),
        (Some(Token::Ident(name)), Some(Token::Op('='))) => {
            let mut tokens = tokens[2..].iter().peekable();
            let expr = expr_bp(&mut tokens, 0);
            if tokens.next().is_some() {
                panic!("Expected EOI, got {:?}", tokens.peek());
            }
            Stmt::Assign(name.clone(), expr)
        }
        _ => {
            let mut tokens = tokens.iter().peekable();
            let expr = expr_bp(&mut tokens, 0);
            if tokens.next().is_some() {
                panic!("Expected EOI, got {:?}", tokens.peek());
            }
            Stmt::Expr(expr)
        }
    }
}

fn expr_bp(tokens: &mut Peekable<Iter<'_, Token>>, min_bp: u8) -> Expr {
    let Some(current) = tokens.next() else {
        panic!("Unexpected EOI");
    };

    // 前置演算子の処理
    let mut lhs = match current.prefix_op() {
        Some((op, r_bp)) => {
            let rhs = expr_bp(tokens, r_bp);
            Expr::Unary(op, Box::new(rhs))
        }
        None => match current {
            Token::Num(x) => Expr::Num(*x),
            Token::Ident(x) => Expr::Ident(x.clone()),
            Token::Op('(') => {
                let res = expr_bp(tokens, 0);
                match tokens.next() {
                    Some(Token::Op(')')) => res,
                    _ => panic!("Expected ')', got {:?}", tokens.peek()),
                }
            }
            Token::Op(op) => panic!("Unexpected op: {:?}", op),
        },
    };

    loop {
        let Some(current) = tokens.peek() else {
            break;
        };

        // 後置演算子の処理
        if let Some((op, l_bp)) = current.postfix_op() {
            if l_bp < min_bp {
                break;
            }
            tokens.next();
            if op == '(' {
                let arg = expr_bp(tokens, 0);
                match tokens.next() {
                    Some(Token::Op(')')) => (),
                    _ => panic!("Expected ')', got {:?}", tokens.peek()),
                }
                let name = match lhs {
                    Expr::Ident(name) => name,
                    _ => panic!("Expected ident, got {:?}", lhs),
                };
                lhs = Expr::Call(name, Box::new(arg));
            } else {
                lhs = Expr::Unary(op, Box::new(lhs));
            }
            continue;
        }

        // 中置演算子の処理
        if let Some((op, l_bp, r_bp)) = current.infix_op() {
            if l_bp < min_bp {
                break;
            }
            tokens.next();
            let rhs = expr_bp(tokens, r_bp);
            lhs = Expr::Binary(op, Box::new(lhs), Box::new(rhs));
            continue;
        }

        break;
    }
    lhs
}

// Priority: (0 is lowest)
//   0: +, -
//   1: *, /
//   2: unary +, unary -
//   3: ^
//   4: !
impl Token {
    fn prefix_op(&self) -> Option<(char, u8)> {
        match self {
            Token::Op(c @ ('+' | '-')) => Some((*c, 5)),
            _ => None,
        }
    }

    fn postfix_op(&self) -> Option<(char, u8)> {
        match self {
            Token::Op(c @ ('!' | '(')) => Some((*c, 9)),
            _ => None,
        }
    }

    fn infix_op(&self) -> Option<(char, u8, u8)> {
        match self {
            Token::Op(c @ ('+' | '-')) => Some((*c, 1, 2)),
            Token::Op(c @ ('*' | '/')) => Some((*c, 3, 4)),
            Token::Op(c @ '^') => Some((*c, 8, 7)),
            _ => None,
        }
    }
}
