mod tokenize;

use std::{iter::Peekable, str::Chars};
use tokenize::{Tokenize, Token};
use crate::path::Path;
use crate::path_builder::PathBuilder;

pub fn parse(str: &str) -> Result<Path, String> {
    let mut chars = str.chars();
    let mut tokens = Tokenize::new(&mut chars).peekable();
    let mut pb = PathBuilder::new();
    while let Some(res) = tokens.next() {
        match res? {
            Token::LargeM => {
                let ns = parse_n_nums(&mut tokens, 2)?;
                pb.move_to(ns[0], ns[1]);
            }
            Token::LargeL => {
                let ns = parse_n_nums(&mut tokens, 2)?;
                pb.line_to(ns[0], ns[1]);
            }
            Token::LargeQ => {
                let ns = parse_n_nums(&mut tokens, 4)?;
                pb.quad(ns[0], ns[1], ns[2], ns[3]);
            }
            Token::LargeC => {
                let ns = parse_n_nums(&mut tokens, 6)?;
                pb.cubic(ns[0], ns[1], ns[2], ns[3], ns[4], ns[5]);
            }
            Token::LargeZ | Token::SmallZ => {
                pb.close();
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
        match tokens.next().unwrap_or(Err(format!("Unexpected EOS")))? {
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
