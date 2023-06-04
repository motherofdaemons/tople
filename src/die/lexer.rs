use std::str::FromStr;

use itertools::Itertools;

use super::dice::Dice;

#[derive(Debug, PartialEq)]
pub struct Lexer {
    tokens: Vec<Token>,
}

#[derive(Debug, PartialEq)]
enum Token {
    Roll(Dice, u32),
    Flat(i32),
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
                            Some(("", sides)) => Ok(Token::Roll(
                                Dice::new(sides.parse::<u32>().map_err(|_| ParseDieFormulaError)?),
                                1,
                            )),
                            Some((rolls, sides)) => Ok(Token::Roll(
                                Dice::new(sides.parse::<u32>().map_err(|_| ParseDieFormulaError)?),
                                rolls.parse::<u32>().map_err(|_| ParseDieFormulaError)?,
                            )),
                            None => Err(ParseDieFormulaError),
                        }
                    } else {
                        Ok(Token::Flat(
                            s.parse::<i32>().map_err(|_| ParseDieFormulaError)?,
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

enum CalcSteps {
    Add,
    Sub,
    Value(i32),
}

impl Lexer {
    #[must_use]
    pub fn calculate(&self) -> i32 {
        let mut total = 0;
        // We want the first value to add so we start the calculator on add
        let mut prev = CalcSteps::Add;
        self.tokens
            .iter()
            .map(|t| match t {
                Token::Add => CalcSteps::Add,
                Token::Sub => CalcSteps::Sub,
                Token::Flat(x) => CalcSteps::Value(*x),
                Token::Roll(d, r) => {
                    // TODO: don't use as because it can die on really large rolls but this is fine
                    // for now
                    CalcSteps::Value((0..*r).map(|_| d.roll() as i32).sum())
                }
            })
            .for_each(|x| {
                if let CalcSteps::Value(v) = x {
                    // TODO: Need to not raw add as it may overflow
                    match prev {
                        CalcSteps::Add => total += v,
                        CalcSteps::Sub => total -= v,
                        // if the prev was a value we don't need to do anything right now
                        CalcSteps::Value(_) => (),
                    }
                };
                prev = x;
            });
        total
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
