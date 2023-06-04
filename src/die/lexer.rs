use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug, PartialEq)]
pub struct Lexer {
    tokens: Vec<Token>,
}

#[derive(Debug, PartialEq)]
enum Token {
    Die(u32, u32),
    Flat(u32),
    Add,
    Sub,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseDieFormulaError;

impl FromStr for Lexer {
    type Err = ParseDieFormulaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Result<Vec<Token>, ParseDieFormulaError> = s
            .split(' ')
            .map(|s| match s {
                "+" => Ok(Token::Add),
                "-" => Ok(Token::Sub),
                s => {
                    if s.contains('d') {
                        match s.split('d').collect_tuple() {
                            Some(("", sides)) => Ok(Token::Die(
                                sides.parse::<u32>().map_err(|_| ParseDieFormulaError)?,
                                1,
                            )),
                            Some((rolls, sides)) => Ok(Token::Die(
                                sides.parse::<u32>().map_err(|_| ParseDieFormulaError)?,
                                rolls.parse::<u32>().map_err(|_| ParseDieFormulaError)?,
                            )),
                            None => Err(ParseDieFormulaError),
                        }
                    } else {
                        Ok(Token::Flat(
                            s.parse::<u32>().map_err(|_| ParseDieFormulaError)?,
                        ))
                    }
                }
            })
            .collect_vec()
            .into_iter()
            .collect();
        Ok(Self { tokens: tokens? })
    }
}

impl Lexer {
    #[must_use]
    pub fn calculate(&self) -> i64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_die_formula() {
        let formula = "d20 + 2d6 - 456".parse::<Lexer>();
        assert_eq!(
            formula,
            Ok(Lexer {
                tokens: vec![
                    Token::Die(20, 1),
                    Token::Add,
                    Token::Die(6, 2),
                    Token::Sub,
                    Token::Flat(456),
                ]
            })
        )
    }
}
