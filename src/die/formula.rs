use std::str::FromStr;

use itertools::Itertools;

use super::dice::Dice;

#[derive(Debug, PartialEq)]

pub struct Formula {
    steps: Vec<FormulaMath>,
}

#[derive(Debug, PartialEq)]
enum FormulaMath {
    AddDie(Dice, u32),
    AddFlat(u32),
    // TODO: Support Subtracting
    // SubDie(Die, u32),
    // SubFlat(u32),
    // TODO: Support keep highest and lowest
    // KeepHighest(idk yet),
    // KeepLowest(idk yet),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseDieFormulaError;

impl FromStr for Formula {
    type Err = ParseDieFormulaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let steps: Result<Vec<FormulaMath>, ParseDieFormulaError> = s
            .split('+')
            .map(|s| match s.split('d').collect::<Vec<_>>()[..] {
                ["", sides] => Ok::<FormulaMath, ParseDieFormulaError>(FormulaMath::AddDie(
                    Dice::new(sides.parse::<u32>().map_err(|_| ParseDieFormulaError)?),
                    1,
                )),
                [num, sides] => Ok::<FormulaMath, ParseDieFormulaError>(FormulaMath::AddDie(
                    Dice::new(sides.parse::<u32>().map_err(|_| ParseDieFormulaError)?),
                    num.parse::<u32>().map_err(|_| ParseDieFormulaError)?,
                )),
                [flat] => Ok::<FormulaMath, ParseDieFormulaError>(FormulaMath::AddFlat(
                    flat.parse::<u32>().map_err(|_| ParseDieFormulaError)?,
                )),
                _ => Err(ParseDieFormulaError),
            })
            .collect_vec()
            .into_iter()
            .collect();
        let steps = steps?;
        Ok(Self { steps })
    }
}

impl Formula {
    #[must_use]
    pub fn calculate(&self) -> i64 {
        self.steps
            .iter()
            .map(|s| match s {
                FormulaMath::AddDie(d, t) => (0..*t).map(|_| i64::from(d.roll())).sum::<i64>(),
                FormulaMath::AddFlat(f) => (*f).into(),
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_die_formula() {
        let formula = "d20+2d6+456".parse::<Formula>();
        assert_eq!(
            formula,
            Ok(Formula {
                steps: vec![
                    FormulaMath::AddDie(Dice::new(20), 1),
                    FormulaMath::AddDie(Dice::new(6), 2),
                    FormulaMath::AddFlat(456),
                ]
            })
        )
    }
}
