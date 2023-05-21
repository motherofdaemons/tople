use std::str::FromStr;

use itertools::Itertools;
use rand::{thread_rng, Rng};

#[derive(Debug, PartialEq)]

pub struct DieFormula {
    steps: Vec<DieMath>,
}

#[derive(Debug, PartialEq)]
enum DieMath {
    Add(Die, u32),
    // Sub(Die, u32),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseDieFormulaError;

impl FromStr for DieFormula {
    type Err = ParseDieFormulaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let steps: Result<Vec<DieMath>, ParseDieFormulaError> = s
            .split('+')
            .map(|s| match s.split('d').collect::<Vec<_>>()[..] {
                ["", sides] => Ok::<DieMath, ParseDieFormulaError>(DieMath::Add(
                    Die::new(sides.parse::<u32>().map_err(|_| ParseDieFormulaError)?),
                    1,
                )),
                [num, sides] => Ok::<DieMath, ParseDieFormulaError>(DieMath::Add(
                    Die::new(sides.parse::<u32>().map_err(|_| ParseDieFormulaError)?),
                    num.parse::<u32>().map_err(|_| ParseDieFormulaError)?,
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

impl DieFormula {
    pub fn calculate(&self) -> i64 {
        self.steps
            .iter()
            .map(|s| match s {
                DieMath::Add(d, t) => (0..*t).map(|_| i64::from(d.roll())).sum::<i64>(),
            })
            .sum()
    }
}

#[derive(Debug, PartialEq)]
pub struct Die {
    sides: u32,
}

impl Die {
    #[must_use]
    pub fn new(sides: u32) -> Self {
        Self { sides }
    }

    #[must_use]
    pub fn roll(&self) -> u32 {
        thread_rng().gen_range(1..=self.sides)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseDieError;

impl FromStr for Die {
    type Err = ParseDieError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(
            s.to_lowercase()
                .trim()
                .strip_prefix('d')
                .ok_or(ParseDieError)?
                .parse::<u32>()
                .map_err(|_| ParseDieError)?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_d2() {
        let d = "d2".parse::<Die>();
        assert!(d == Ok(Die::new(2)))
    }
    #[test]
    fn parse_d4() {
        let d = "d4".parse::<Die>();
        assert!(d == Ok(Die::new(4)))
    }
    #[test]
    fn parse_d6() {
        let d = "d6".parse::<Die>();
        assert!(d == Ok(Die::new(6)))
    }

    #[test]
    fn parse_d8() {
        let d = "d8".parse::<Die>();
        assert!(d == Ok(Die::new(8)))
    }

    #[test]
    fn parse_d10() {
        let d = "d10".parse::<Die>();
        assert!(d == Ok(Die::new(10)))
    }
    #[test]
    fn parse_d12() {
        let d = "d12".parse::<Die>();
        assert!(d == Ok(Die::new(12)))
    }
    #[test]
    fn parse_d20() {
        let d = "d20".parse::<Die>();
        assert!(d == Ok(Die::new(20)))
    }
    #[test]
    fn parse_d100() {
        let d = "d100".parse::<Die>();
        assert!(d == Ok(Die::new(100)))
    }
    #[test]
    fn parse_die_error() {
        let d = "asd".parse::<Die>();
        assert!(d == Err(ParseDieError))
    }
    #[test]
    fn parse_die_formula() {
        let formula = "d20+d20".parse::<DieFormula>();
        assert_eq!(
            formula,
            Ok(DieFormula {
                steps: vec![DieMath::Add(Die::new(20), 1), DieMath::Add(Die::new(20), 1)]
            })
        )
    }
}
