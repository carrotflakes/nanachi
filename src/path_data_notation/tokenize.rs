use std::{iter::Peekable, num::ParseFloatError};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LargeM,
    SmallM,
    LargeL,
    SmallL,
    LargeH,
    SmallH,
    LargeV,
    SmallV,
    LargeZ,
    SmallZ,
    LargeC,
    SmallC,
    LargeS,
    SmallS,
    LargeQ,
    SmallQ,
    LargeT,
    SmallT,
    LargeA,
    SmallA,
    Num(f32),
    Comma,
    EOS,
}

pub struct Tokenize<'a, T: Iterator<Item = char>> {
    chars: Peekable<&'a mut T>,
    ended: bool,
}

impl<'a, T: Iterator<Item = char>> Tokenize<'a, T> {
    pub fn new(it: &'a mut T) -> Tokenize<'a, T> {
        Tokenize {
            chars: it.peekable(),
            ended: false,
        }
    }

    pub fn try_parse_num(&mut self) -> Option<Result<f32, String>> {
        let mut str = if let Some('-') = self.chars.peek() {
            self.chars.next();
            "-"
        } else {
            ""
        }
        .to_string();
        let mut dot = false;
        loop {
            match self.chars.peek().copied() {
                Some(c) if c.is_numeric() || c == '.' => {
                    if c == '.' {
                        if dot {
                            break;
                        }
                        dot = true;
                    }
                    self.chars.next();
                    str.push(c);
                }
                _ => {
                    break;
                }
            }
        }
        if str.is_empty() {
            None
        } else {
            Some(
                str.parse()
                    .map_err(|e: ParseFloatError| format!("{}: {:?}", e, str)),
            )
        }
    }
}

impl<'a, T: Iterator<Item = char>> Iterator for Tokenize<'a, T> {
    type Item = Result<Token, String>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(c) = self.chars.peek() {
            if c.is_ascii_whitespace() {
                self.chars.next();
            } else {
                break;
            }
        }

        if let None = self.chars.peek() {
            if self.ended {
                return None;
            } else {
                self.ended = true;
                return Some(Ok(Token::EOS));
            }
        }

        match self.try_parse_num() {
            Some(Ok(n)) => {
                return Some(Ok(Token::Num(n)));
            }
            Some(Err(string)) => {
                return Some(Err(string));
            }
            None => {}
        }

        match self.chars.peek() {
            Some(',') => {
                self.chars.next();
                Some(Ok(Token::Comma))
            }
            Some('M') => {
                self.chars.next();
                Some(Ok(Token::LargeM))
            }
            Some('m') => {
                self.chars.next();
                Some(Ok(Token::SmallM))
            }
            Some('L') => {
                self.chars.next();
                Some(Ok(Token::LargeL))
            }
            Some('l') => {
                self.chars.next();
                Some(Ok(Token::SmallL))
            }
            Some('H') => {
                self.chars.next();
                Some(Ok(Token::LargeH))
            }
            Some('h') => {
                self.chars.next();
                Some(Ok(Token::SmallH))
            }
            Some('V') => {
                self.chars.next();
                Some(Ok(Token::LargeV))
            }
            Some('v') => {
                self.chars.next();
                Some(Ok(Token::SmallV))
            }
            Some('Z') => {
                self.chars.next();
                Some(Ok(Token::LargeZ))
            }
            Some('z') => {
                self.chars.next();
                Some(Ok(Token::SmallZ))
            }
            Some('C') => {
                self.chars.next();
                Some(Ok(Token::LargeC))
            }
            Some('c') => {
                self.chars.next();
                Some(Ok(Token::SmallC))
            }
            Some('S') => {
                self.chars.next();
                Some(Ok(Token::LargeS))
            }
            Some('s') => {
                self.chars.next();
                Some(Ok(Token::SmallS))
            }
            Some('Q') => {
                self.chars.next();
                Some(Ok(Token::LargeQ))
            }
            Some('q') => {
                self.chars.next();
                Some(Ok(Token::SmallQ))
            }
            Some('T') => {
                self.chars.next();
                Some(Ok(Token::LargeT))
            }
            Some('t') => {
                self.chars.next();
                Some(Ok(Token::SmallT))
            }
            Some('A') => {
                self.chars.next();
                Some(Ok(Token::LargeA))
            }
            Some('a') => {
                self.chars.next();
                Some(Ok(Token::SmallA))
            }
            Some(c) => Some(Err(format!("Unexpected char: {}", c))),
            None => None,
        }
    }
}

#[test]
fn test() {
    let src = "M 10 20 30.0,40.5.123";
    let mut chars = src.chars();
    let tokens = Tokenize::new(&mut chars);
    dbg!(tokens.collect::<Vec<_>>());
}
