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
