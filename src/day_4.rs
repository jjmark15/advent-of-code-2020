use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

use crate::day_4::EyeColour::{Amber, Blue, Brown, Green, Grey, Hazel, Other};
use crate::day_4::HeightUnit::{Centimetres, Inches};

#[allow(dead_code)]
struct RelaxedValidationPassport {
    ecl: String,
    pid: String,
    eyr: String,
    hcl: String,
    byr: String,
    iyr: String,
    cid: Option<String>,
    hgt: String,
}

impl RelaxedValidationPassport {
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
        RelaxedValidationPassport {
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

impl FromStr for RelaxedValidationPassport {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let missing_required_field_error_message = "Could not find required field in passport text";
        let passport_reader = PassportReader::new(s.to_string());

        Ok(RelaxedValidationPassport::new(
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

#[allow(dead_code)]
struct HairColour {
    code: String,
}

impl FromStr for HairColour {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        }

        if RE.is_match(s) {
            Ok(HairColour {
                code: s.to_string(),
            })
        } else {
            Err(anyhow::Error::msg("Invalid hair colour code"))
        }
    }
}

enum EyeColour {
    Amber,
    Blue,
    Brown,
    Grey,
    Green,
    Hazel,
    Other,
}

impl FromStr for EyeColour {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "amb" => Ok(Amber),
            "blu" => Ok(Blue),
            "brn" => Ok(Brown),
            "gry" => Ok(Grey),
            "grn" => Ok(Green),
            "hzl" => Ok(Hazel),
            "oth" => Ok(Other),
            _ => Err(anyhow::Error::msg("Invalid eye colour")),
        }
    }
}

enum HeightUnit {
    Inches,
    Centimetres,
}

impl FromStr for HeightUnit {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "in" => Ok(Inches),
            "cm" => Ok(Centimetres),
            _ => Err(anyhow::Error::msg("Invalid unit of height")),
        }
    }
}

#[allow(dead_code)]
struct Height {
    value: u32,
    unit: HeightUnit,
}

impl Height {
    fn is_valid(&self) -> bool {
        match self.unit {
            HeightUnit::Centimetres => self.value >= 150 && self.value <= 193,
            HeightUnit::Inches => self.value >= 59 && self.value <= 76,
        }
    }
}

impl FromStr for Height {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+)(\w+)$").unwrap();
        }

        match RE.captures(s) {
            Some(captures) => Ok(Height {
                value: captures.get(1).unwrap().as_str().parse()?,
                unit: captures.get(2).unwrap().as_str().parse()?,
            }),
            None => Err(anyhow::Error::msg("Invalid height string")),
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Year {
    year: u32,
}

impl Year {
    fn new(year: u32) -> Self {
        Year { year }
    }
}

impl FromStr for Year {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\d{4}$").unwrap();
        }

        match RE.is_match(s) {
            true => Ok(Year { year: s.parse()? }),
            false => Err(anyhow::Error::msg("Invalid year string")),
        }
    }
}

#[allow(dead_code)]
struct PassportId {
    value: String,
}

impl FromStr for PassportId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\d{9}$").unwrap();
        }

        match RE.is_match(s) {
            true => Ok(PassportId {
                value: s.to_string(),
            }),
            false => Err(anyhow::Error::msg("Invalid Passport ID")),
        }
    }
}

#[allow(dead_code)]
struct StrictValidationPassport {
    ecl: EyeColour,
    pid: PassportId,
    eyr: Year,
    hcl: HairColour,
    byr: Year,
    iyr: Year,
    cid: Option<String>,
    hgt: Height,
}

impl StrictValidationPassport {
    fn new(
        ecl: EyeColour,
        pid: PassportId,
        eyr: Year,
        hcl: HairColour,
        byr: Year,
        iyr: Year,
        cid: Option<String>,
        hgt: Height,
    ) -> Self {
        StrictValidationPassport {
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

    fn is_valid(&self) -> bool {
        self.byr >= Year::new(1920)
            && self.byr <= Year::new(2002)
            && self.iyr >= Year::new(2010)
            && self.iyr <= Year::new(2020)
            && self.eyr >= Year::new(2020)
            && self.eyr <= Year::new(2030)
            && self.hgt.is_valid()
    }
}

impl FromStr for StrictValidationPassport {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let missing_required_field_error_message = "Could not find required field in passport text";
        let passport_reader = PassportReader::new(s.to_string());

        Ok(StrictValidationPassport::new(
            passport_reader
                .get_field("ecl")
                .ok_or_else(|| anyhow::Error::msg(missing_required_field_error_message))?
                .parse()?,
            passport_reader
                .get_field("pid")
                .ok_or_else(|| anyhow::Error::msg(missing_required_field_error_message))?
                .parse()?,
            passport_reader
                .get_field("eyr")
                .ok_or_else(|| anyhow::Error::msg(missing_required_field_error_message))?
                .parse()?,
            passport_reader
                .get_field("hcl")
                .ok_or_else(|| anyhow::Error::msg(missing_required_field_error_message))?
                .parse()?,
            passport_reader
                .get_field("byr")
                .ok_or_else(|| anyhow::Error::msg(missing_required_field_error_message))?
                .parse()?,
            passport_reader
                .get_field("iyr")
                .ok_or_else(|| anyhow::Error::msg(missing_required_field_error_message))?
                .parse()?,
            passport_reader.get_field("cid"),
            passport_reader
                .get_field("hgt")
                .ok_or_else(|| anyhow::Error::msg(missing_required_field_error_message))?
                .parse()?,
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

pub fn count_valid_relaxed_validation_passports_in_text(passport_strings: Vec<String>) -> usize {
    passport_strings
        .iter()
        .filter(|passport_string| {
            RelaxedValidationPassport::from_str(passport_string.as_str()).is_ok()
        })
        .count()
}

pub fn count_valid_strict_validation_passports_in_text(passport_strings: Vec<String>) -> usize {
    passport_strings
        .iter()
        .filter(|passport_string| {
            let passport = StrictValidationPassport::from_str(passport_string.as_str());
            passport.is_ok() && passport.unwrap().is_valid()
        })
        .count()
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use super::*;

    #[test]
    fn counts_valid_relaxed_validation_passports() {
        let passport_strings: Vec<String> = vec![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm",
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929",
            "hcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm",
            "hcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in",
        ]
        .iter()
        .map(ToString::to_string)
        .collect();

        assert_that(&count_valid_relaxed_validation_passports_in_text(
            passport_strings,
        ))
        .is_equal_to(2);
    }

    #[test]
    fn recognises_invalid_strict_validation_passports() {
        let passport_strings: Vec<String> = vec![
            "eyr:1972 cid:100\nhcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
            "iyr:2019\nhcl:#602927 eyr:1967 hgt:170cm\necl:grn pid:012533040 byr:1946",
            "hcl:dab227 iyr:2012\necl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
            "hgt:59cm ecl:zzz\neyr:2038 hcl:74454a iyr:2023\npid:3556412378 byr:2007",
        ]
        .iter()
        .map(ToString::to_string)
        .collect();

        assert_that(&count_valid_strict_validation_passports_in_text(
            passport_strings,
        ))
        .is_equal_to(0);
    }

    #[test]
    fn recognises_valid_strict_validation_passports() {
        let passport_strings: Vec<String> = vec![
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\nhcl:#623a2f",
            "eyr:2029 ecl:blu cid:129 byr:1989\niyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
            "hcl:#888785\nhgt:164cm byr:2001 iyr:2015 cid:88\npid:545766238 ecl:hzl\neyr:2022",
            "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
        ]
        .iter()
        .map(ToString::to_string)
        .collect();

        assert_that(&count_valid_strict_validation_passports_in_text(
            passport_strings,
        ))
        .is_equal_to(4);
    }
}
