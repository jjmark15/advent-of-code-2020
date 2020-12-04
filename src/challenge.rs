use std::str::FromStr;

use regex::Regex;

use crate::challenge::ChallengePart::{One, Two};

#[derive(Debug, Copy, Clone)]
pub struct Challenge {
    day: u8,
    part: ChallengePart,
}

impl Challenge {
    pub fn day(&self) -> u8 {
        self.day
    }

    pub fn part(&self) -> ChallengePart {
        self.part
    }
}

impl FromStr for Challenge {
    type Err = ChallengeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(\d)\.(\d)$").unwrap();
        match re.captures(s) {
            Some(captures) => Ok(Challenge {
                day: captures.get(1).unwrap().as_str().parse().unwrap(),
                part: captures.get(2).unwrap().as_str().parse().unwrap(),
            }),
            None => Err(ChallengeParseError),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Could not parse challenge")]
pub struct ChallengeParseError;

#[derive(Debug, Copy, Clone)]
pub enum ChallengePart {
    One,
    Two,
}

impl FromStr for ChallengePart {
    type Err = ChallengeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u8>() {
            Ok(n) => {
                if n == 1 {
                    Ok(One)
                } else if n == 2 {
                    Ok(Two)
                } else {
                    Err(ChallengeParseError)
                }
            }
            Err(_) => Err(ChallengeParseError),
        }
    }
}
