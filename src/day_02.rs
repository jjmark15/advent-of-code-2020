use std::convert::Infallible;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref POLICY_REGEX: Regex = Regex::new(r"^(\d+)-(\d+) (\w)$").unwrap();
}

pub trait PasswordPolicy {
    fn is_satisfied_by(&self, password: &Password) -> bool;
}

#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
pub struct OccurrenceRestrictedPasswordPolicy {
    restricted_character: char,
    minimum_occurrence: u32,
    maximum_occurrence: u32,
}

impl OccurrenceRestrictedPasswordPolicy {
    pub fn new(
        restricted_character: char,
        minimum_occurrence: u32,
        maximum_occurrence: u32,
    ) -> Self {
        OccurrenceRestrictedPasswordPolicy {
            restricted_character,
            minimum_occurrence,
            maximum_occurrence,
        }
    }
}

impl PasswordPolicy for OccurrenceRestrictedPasswordPolicy {
    fn is_satisfied_by(&self, password: &Password) -> bool {
        let occurrences = password
            .value()
            .chars()
            .filter(|&c| c == self.restricted_character)
            .count() as u32;

        occurrences >= self.minimum_occurrence && occurrences <= self.maximum_occurrence
    }
}

impl FromStr for OccurrenceRestrictedPasswordPolicy {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // lazy_static! {
        //     static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) (\w)$").unwrap();
        // }

        match POLICY_REGEX.captures(s) {
            Some(captures) => Ok(OccurrenceRestrictedPasswordPolicy::new(
                captures.get(3).unwrap().as_str().parse()?,
                captures.get(1).unwrap().as_str().parse()?,
                captures.get(2).unwrap().as_str().parse()?,
            )),
            None => Err(anyhow::Error::msg("could not parse Password Policy")),
        }
    }
}

#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
pub struct PositionallyRestrictedPasswordPolicy {
    restricted_character: char,
    first_position: usize,
    second_position: usize,
}

impl PositionallyRestrictedPasswordPolicy {
    pub fn new(restricted_character: char, first_position: usize, second_position: usize) -> Self {
        PositionallyRestrictedPasswordPolicy {
            restricted_character,
            first_position,
            second_position,
        }
    }

    fn character_at_position_equals_restricted(
        &self,
        position: usize,
        password: &Password,
    ) -> bool {
        match password.value().get((position - 1)..position) {
            Some(s) => s.chars().next().unwrap() == self.restricted_character,
            None => false,
        }
    }
}

impl PasswordPolicy for PositionallyRestrictedPasswordPolicy {
    fn is_satisfied_by(&self, password: &Password) -> bool {
        if self.character_at_position_equals_restricted(self.first_position, password) {
            if !self.character_at_position_equals_restricted(self.second_position, password) {
                return true;
            }
        } else if self.character_at_position_equals_restricted(self.second_position, password) {
            return true;
        }
        false
    }
}

impl FromStr for PositionallyRestrictedPasswordPolicy {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match POLICY_REGEX.captures(s) {
            Some(captures) => Ok(PositionallyRestrictedPasswordPolicy::new(
                captures.get(3).unwrap().as_str().parse()?,
                captures.get(1).unwrap().as_str().parse()?,
                captures.get(2).unwrap().as_str().parse()?,
            )),
            None => Err(anyhow::Error::msg("could not parse Password Policy")),
        }
    }
}

#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
pub struct Password {
    value: String,
}

impl Password {
    pub fn new(value: String) -> Self {
        Password { value }
    }

    pub fn value(&self) -> &String {
        &self.value
    }
}

impl FromStr for Password {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Password::new(s.to_string()))
    }
}

pub fn to_policy_and_password<
    S: AsRef<str>,
    Policy: PasswordPolicy + FromStr<Err = anyhow::Error>,
>(
    s: S,
) -> anyhow::Result<(Policy, Password)> {
    let splits: Vec<&str> = s.as_ref().split(": ").take(2).collect();
    let policy: Policy = splits.get(0).unwrap().parse()?;
    let password: Password = splits.get(1).unwrap().parse()?;

    Ok((policy, password))
}

pub fn count_policies_satisfied_by_passwords<Policy: PasswordPolicy>(
    policies_and_passwords: Vec<(Policy, Password)>,
) -> usize {
    policies_and_passwords
        .iter()
        .filter(|(policy, password)| policy.is_satisfied_by(password))
        .count()
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use super::*;

    #[test]
    fn finds_occurrence_valid_passwords() {
        let policies_and_passwords: Vec<(OccurrenceRestrictedPasswordPolicy, Password)> = vec![
            (
                OccurrenceRestrictedPasswordPolicy::new('a', 1, 3),
                Password::new("abcde".to_string()),
            ),
            (
                OccurrenceRestrictedPasswordPolicy::new('b', 1, 3),
                Password::new("cdefg".to_string()),
            ),
            (
                OccurrenceRestrictedPasswordPolicy::new('c', 2, 9),
                Password::new("ccccccccc".to_string()),
            ),
        ];

        assert_that(&count_policies_satisfied_by_passwords(
            policies_and_passwords,
        ))
        .is_equal_to(2)
    }

    #[test]
    fn finds_positionally_valid_passwords() {
        let policies_and_passwords = vec![
            (
                PositionallyRestrictedPasswordPolicy::new('a', 1, 3),
                Password::new("abcde".to_string()),
            ),
            (
                PositionallyRestrictedPasswordPolicy::new('b', 1, 3),
                Password::new("cdefg".to_string()),
            ),
            (
                PositionallyRestrictedPasswordPolicy::new('c', 2, 9),
                Password::new("ccccccccc".to_string()),
            ),
        ];

        assert_that(&count_policies_satisfied_by_passwords(
            policies_and_passwords,
        ))
        .is_equal_to(1)
    }

    #[test]
    fn converts_string_to_occurrence_restricted_policy_and_password() {
        let result = to_policy_and_password("1-3 a: abcde").unwrap();

        assert_that(&result.0).is_equal_to(OccurrenceRestrictedPasswordPolicy::new('a', 1, 3));
        assert_that(&result.1).is_equal_to(Password::new("abcde".to_string()));
    }

    #[test]
    fn converts_string_to_position_restricted_policy_and_password() {
        let result = to_policy_and_password("1-3 a: abcde").unwrap();

        assert_that(&result.0).is_equal_to(PositionallyRestrictedPasswordPolicy::new('a', 1, 3));
        assert_that(&result.1).is_equal_to(Password::new("abcde".to_string()));
    }
}
