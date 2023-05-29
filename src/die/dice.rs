use std::str::FromStr;

use rand::{thread_rng, Rng};

#[derive(Debug, PartialEq)]
pub struct Dice {
    sides: u32,
}

impl Dice {
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

impl FromStr for Dice {
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
        let d = "d2".parse::<Dice>();
        assert!(d == Ok(Dice::new(2)))
    }
    #[test]
    fn parse_d4() {
        let d = "d4".parse::<Dice>();
        assert!(d == Ok(Dice::new(4)))
    }
    #[test]
    fn parse_d6() {
        let d = "d6".parse::<Dice>();
        assert!(d == Ok(Dice::new(6)))
    }

    #[test]
    fn parse_d8() {
        let d = "d8".parse::<Dice>();
        assert!(d == Ok(Dice::new(8)))
    }

    #[test]
    fn parse_d10() {
        let d = "d10".parse::<Dice>();
        assert!(d == Ok(Dice::new(10)))
    }
    #[test]
    fn parse_d12() {
        let d = "d12".parse::<Dice>();
        assert!(d == Ok(Dice::new(12)))
    }
    #[test]
    fn parse_d20() {
        let d = "d20".parse::<Dice>();
        assert!(d == Ok(Dice::new(20)))
    }
    #[test]
    fn parse_d100() {
        let d = "d100".parse::<Dice>();
        assert!(d == Ok(Dice::new(100)))
    }
    #[test]
    fn parse_die_error() {
        let d = "asd".parse::<Dice>();
        assert!(d == Err(ParseDieError))
    }
}
