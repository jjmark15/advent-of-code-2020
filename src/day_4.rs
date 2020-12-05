use regex::Regex;
use std::str::FromStr;

#[allow(dead_code)]
struct Passport {
    ecl: String,
    pid: String,
    eyr: String,
    hcl: String,
    byr: String,
    iyr: String,
    cid: Option<String>,
    hgt: String,
}

impl Passport {
    fn new(
        ecl: String,
        pid: String,
        eyr: String,
        hcl: String,
        byr: String,
        iyr: String,
        cid: Option<String>,
        hgt: String,
    ) -> Self {
        Passport {
            ecl,
            pid,
            eyr,
            hcl,
            byr,
            iyr,
            cid,
            hgt,
        }
    }
}

impl FromStr for Passport {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let missing_required_field_error_message = "Could not find required field in passport text";
        let passport_reader = PassportReader::new(s.to_string());

        Ok(Passport::new(
            passport_reader
                .get_field("ecl")
                .ok_or_else(|| anyhow::Error::msg(missing_required_field_error_message))?,
            passport_reader
                .get_field("pid")
                .ok_or_else(|| anyhow::Error::msg(missing_required_field_error_message))?,
            passport_reader
                .get_field("eyr")
                .ok_or_else(|| anyhow::Error::msg(missing_required_field_error_message))?,
            passport_reader
                .get_field("hcl")
                .ok_or_else(|| anyhow::Error::msg(missing_required_field_error_message))?,
            passport_reader
                .get_field("byr")
                .ok_or_else(|| anyhow::Error::msg(missing_required_field_error_message))?,
            passport_reader
                .get_field("iyr")
                .ok_or_else(|| anyhow::Error::msg(missing_required_field_error_message))?,
            passport_reader.get_field("cid"),
            passport_reader
                .get_field("hgt")
                .ok_or_else(|| anyhow::Error::msg(missing_required_field_error_message))?,
        ))
    }
}

struct PassportReader {
    text: String,
}

impl PassportReader {
    fn new(text: String) -> Self {
        PassportReader { text }
    }

    fn get_field(&self, field_name: &str) -> Option<String> {
        let re = Regex::new(format!(r"{}:(\S+)", field_name).as_str()).unwrap();
        match re.captures(self.text.as_str()) {
            Some(captures) => Some(captures.get(1).unwrap().as_str().to_string()),
            None => None,
        }
    }
}

pub fn count_valid_passports_in_text(passport_strings: Vec<String>) -> usize {
    passport_strings
        .iter()
        .filter(|passport_string| Passport::from_str(passport_string.as_str()).is_ok())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn counts_valid_passports() {
        let passport_strings: Vec<String> = vec![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm",
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929",
            "hcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm",
            "hcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in",
        ]
        .iter()
        .map(ToString::to_string)
        .collect();

        assert_that(&count_valid_passports_in_text(passport_strings)).is_equal_to(2);
    }
}
