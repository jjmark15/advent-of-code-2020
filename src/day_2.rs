use std::convert::Infallible;
use std::str::FromStr;

use regex::Regex;

#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
pub struct PasswordPolicy {
    restricted_character: char,
    minimum_occurrence: u32,
    maximum_occurrence: u32,
}

impl PasswordPolicy {
    pub fn new(
        restricted_character: char,
        minimum_occurrence: u32,
        maximum_occurrence: u32,
    ) -> Self {
        PasswordPolicy {
            restricted_character,
            minimum_occurrence,
            maximum_occurrence,
        }
    }

    pub fn is_satisfied_by(&self, password: &Password) -> bool {
        let occurrences = password
            .value()
            .chars()
            .filter(|&c| c == self.restricted_character)
            .count() as u32;

        occurrences >= self.minimum_occurrence && occurrences <= self.maximum_occurrence
    }
}

impl FromStr for PasswordPolicy {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(r"^(\d+)-(\d+) (\w)$").unwrap();

        match regex.captures(s) {
            Some(captures) => Ok(PasswordPolicy::new(
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

pub fn to_policy_and_password<S: AsRef<str>>(s: S) -> anyhow::Result<(PasswordPolicy, Password)> {
    let splits: Vec<&str> = s.as_ref().split(": ").take(2).collect();
    let policy: PasswordPolicy = splits.get(0).unwrap().parse()?;
    let password: Password = splits.get(1).unwrap().parse()?;

    Ok((policy, password))
}

pub fn count_policies_satisfied_by_passwords(
    policies_and_passwords: Vec<(PasswordPolicy, Password)>,
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
    fn finds_valid_passwords() {
        let policies_and_passwords: Vec<(PasswordPolicy, Password)> = vec![
            (
                PasswordPolicy::new('a', 1, 3),
                Password::new("abcde".to_string()),
            ),
            (
                PasswordPolicy::new('b', 1, 3),
                Password::new("cdefg".to_string()),
            ),
            (
                PasswordPolicy::new('c', 2, 9),
                Password::new("ccccccccc".to_string()),
            ),
        ];

        assert_that(&count_policies_satisfied_by_passwords(
            policies_and_passwords,
        ))
        .is_equal_to(2)
    }

    #[test]
    fn converts_string_to_policy_and_password() {
        let result = to_policy_and_password("1-3 a: abcde").unwrap();

        assert_that(&result.0).is_equal_to(PasswordPolicy::new('a', 1, 3));
        assert_that(&result.1).is_equal_to(Password::new("abcde".to_string()));
    }
}
