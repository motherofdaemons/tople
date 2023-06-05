use itertools::Itertools;

use super::lexer::{Lexer, Token};

#[derive(Copy, Clone)]
enum CalcSteps {
    Add,
    Sub,
    Value(i32),
}

pub struct Parser {
    steps: Vec<CalcSteps>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseDieFormulaError;

impl Parser {
    pub fn new(lexer: &Lexer) -> Result<Self, ParseDieFormulaError> {
        Ok(Self {
            steps: lexer
                .tokens
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
                .collect_vec(),
        })
    }

    #[must_use]
    pub fn parse(self) -> i32 {
        let mut total = 0;
        // We want the first value to add so we start the calculator on add
        let mut prev = CalcSteps::Add;
        self.steps.iter().for_each(|x| {
            if let CalcSteps::Value(v) = x {
                // TODO: Need to not raw add as it may overflow
                match prev {
                    CalcSteps::Add => total += v,
                    CalcSteps::Sub => total -= v,
                    // if the prev was a value we don't need to do anything right now
                    CalcSteps::Value(_) => (),
                }
            };
            prev = *x;
        });
        total
    }
}
