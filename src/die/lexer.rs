use itertools::Itertools;

use super::dice::Dice;

#[derive(Debug, PartialEq)]
pub struct Lexer {
    pub tokens: Vec<Token>,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Roll(Dice, u32),
    Flat(i32),
    Add,
    Sub,
}

#[derive(Debug, PartialEq, Eq)]
pub struct LexDieFormulaError;

impl Lexer {
    pub fn new(s: &str) -> Result<Self, LexDieFormulaError> {
        let tokens: Result<Vec<Token>, LexDieFormulaError> = s
            .split(' ')
            .map(|s| match s {
                "+" => Ok(Token::Add),
                "-" => Ok(Token::Sub),
                s => {
                    if s.contains('d') {
                        match s.split('d').collect_tuple() {
                            Some(("", sides)) => Ok(Token::Roll(
                                Dice::new(sides.parse::<u32>().map_err(|_| LexDieFormulaError)?),
                                1,
                            )),
                            Some((rolls, sides)) => Ok(Token::Roll(
                                Dice::new(sides.parse::<u32>().map_err(|_| LexDieFormulaError)?),
                                rolls.parse::<u32>().map_err(|_| LexDieFormulaError)?,
                            )),
                            None => Err(LexDieFormulaError),
                        }
                    } else {
                        Ok(Token::Flat(
                            s.parse::<i32>().map_err(|_| LexDieFormulaError)?,
                        ))
                    }
                }
            })
            .collect_vec()
            .into_iter()
            .collect();
        // TODO: validate that the tokens are valid by having value operation value so on and so on
        Ok(Self { tokens: tokens? })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_die_formula() {
        let lexer = Lexer::new("d20 + 2d6 - 456");
        assert_eq!(
            lexer,
            Ok(Lexer {
                tokens: vec![
                    Token::Roll(Dice::new(20), 1),
                    Token::Add,
                    Token::Roll(Dice::new(6), 2),
                    Token::Sub,
                    Token::Flat(456),
                ]
            })
        )
    }
}
