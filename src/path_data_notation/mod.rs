mod tokenize;

use std::{iter::Peekable, str::Chars};
use tokenize::{Tokenize, Token};
use crate::path::Path;
use crate::path_builder::PathBuilder;

pub fn parse(str: &str) -> Result<Path, String> {
    let mut chars = str.chars();
    let mut tokens = Tokenize::new(&mut chars).peekable();
    let mut pb = PathBuilder::new();
    loop {
        match tokens.next().unwrap()? {
            Token::LargeM => {
                let ns = parse_n_nums(&mut tokens, 2)?;
                pb.move_to(ns[0], ns[1]);
                while is_num_token(&tokens.peek().unwrap().to_owned()?) {
                    let ns = parse_n_nums(&mut tokens, 2)?;
                    pb.line_to(ns[0], ns[1]);
                }
            }
            Token::SmallM => {
                let ns = parse_n_nums(&mut tokens, 2)?;
                let current = pb.current_pos().unwrap_or((0.0, 0.0));
                pb.move_to(current.0 + ns[0], current.1 + ns[1]);
                while is_num_token(&tokens.peek().unwrap().to_owned()?) {
                    let ns = parse_n_nums(&mut tokens, 2)?;
                    let current = pb.current_pos().unwrap();
                    pb.line_to(current.0 + ns[0], current.1 + ns[1]);
                }
            }
            Token::LargeL => {
                let ns = parse_n_nums(&mut tokens, 2)?;
                pb.line_to(ns[0], ns[1]);
                while is_num_token(&tokens.peek().unwrap().to_owned()?) {
                    let ns = parse_n_nums(&mut tokens, 2)?;
                    pb.line_to(ns[0], ns[1]);
                }
            }
            Token::SmallL => {
                let ns = parse_n_nums(&mut tokens, 2)?;
                let current = pb.current_pos().unwrap_or((0.0, 0.0));
                pb.line_to(current.0 + ns[0], current.1 + ns[1]);
                while is_num_token(&tokens.peek().unwrap().to_owned()?) {
                    let ns = parse_n_nums(&mut tokens, 2)?;
                    let current = pb.current_pos().unwrap();
                    pb.line_to(current.0 + ns[0], current.1 + ns[1]);
                }
            }
            Token::LargeH => {
                let ns = parse_n_nums(&mut tokens, 1)?;
                let current = pb.current_pos().unwrap_or((0.0, 0.0));
                pb.line_to(ns[0], current.1);
                while is_num_token(&tokens.peek().unwrap().to_owned()?) {
                    let ns = parse_n_nums(&mut tokens, 1)?;
                    let current = pb.current_pos().unwrap_or((0.0, 0.0));
                    pb.line_to(ns[0], current.1);
                }
            }
            Token::SmallH => {
                let ns = parse_n_nums(&mut tokens, 1)?;
                let current = pb.current_pos().unwrap_or((0.0, 0.0));
                pb.line_to(current.0 + ns[0], current.1);
                while is_num_token(&tokens.peek().unwrap().to_owned()?) {
                    let ns = parse_n_nums(&mut tokens, 1)?;
                    let current = pb.current_pos().unwrap_or((0.0, 0.0));
                    pb.line_to(current.0 + ns[0], current.1);
                }
            }
            Token::LargeV => {
                let ns = parse_n_nums(&mut tokens, 1)?;
                let current = pb.current_pos().unwrap_or((0.0, 0.0));
                pb.line_to(current.0, ns[0]);
                while is_num_token(&tokens.peek().unwrap().to_owned()?) {
                    let ns = parse_n_nums(&mut tokens, 1)?;
                    let current = pb.current_pos().unwrap_or((0.0, 0.0));
                    pb.line_to(current.0, ns[0]);
                }
            }
            Token::SmallV => {
                let ns = parse_n_nums(&mut tokens, 1)?;
                let current = pb.current_pos().unwrap_or((0.0, 0.0));
                pb.line_to(current.0, current.1 + ns[0]);
                while is_num_token(&tokens.peek().unwrap().to_owned()?) {
                    let ns = parse_n_nums(&mut tokens, 1)?;
                    let current = pb.current_pos().unwrap_or((0.0, 0.0));
                    pb.line_to(current.0, current.1 + ns[0]);
                }
            }
            Token::LargeQ => {
                let ns = parse_n_nums(&mut tokens, 4)?;
                pb.quad(ns[0], ns[1], ns[2], ns[3]);
            }
            // TODO T
            Token::LargeC => {
                let ns = parse_n_nums(&mut tokens, 6)?;
                pb.cubic(ns[0], ns[1], ns[2], ns[3], ns[4], ns[5]);
            }
            // TODO S, A
            Token::LargeZ | Token::SmallZ => {
                pb.close();
            }
            Token::EOS => {
                break;
            }
            token => {
                return Err(format!("Unexpected token: {:?}", token));
            }
        }
    }
    Ok(pb.end())
}

fn parse_n_nums(tokens: &mut Peekable<Tokenize<Chars>>, n: usize) -> Result<Vec<f64>, String> {
    let mut v = Vec::new();
    for _ in 0..n {
        skip_comma(tokens)?;
        match tokens.next().unwrap()? {
            Token::Num(n) => {
                v.push(n);
            }
            token => {
                return Err(format!("Expects num, but found: {:?}", token));
            }
        };
    }
    Ok(v)
}

fn is_num_token(token: &Token) -> bool {
    match token {
        Token::Num(_) => true,
        _ => false,
    }
}

fn skip_comma(tokens: &mut Peekable<Tokenize<Chars>>) -> Result<(), String> {
    match tokens.peek().unwrap()? {
        Token::Comma => {
            tokens.next().unwrap()?;
        }
        _ => {}
    }
}
