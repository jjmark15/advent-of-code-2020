use std::num::ParseIntError;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

struct NumberRange {
    lower: u64,
    upper: u64,
}

impl NumberRange {
    fn new(lower: u64, upper: u64) -> Self {
        NumberRange { lower, upper }
    }

    fn lower(&self) -> u64 {
        self.lower
    }

    fn upper(&self) -> u64 {
        self.upper
    }
}

struct Ticket {
    field_values: Vec<u64>,
}

impl Ticket {
    fn new(field_values: Vec<u64>) -> Self {
        Ticket { field_values }
    }

    fn field_values(&self) -> &Vec<u64> {
        &self.field_values
    }
}

struct TicketFieldRule {
    _name: String,
    number_ranges: Vec<NumberRange>,
}

impl TicketFieldRule {
    fn satisfied_by(&self, n: u64) -> bool {
        for range in &self.number_ranges {
            if n >= range.lower() && n <= range.upper() {
                return true;
            }
        }

        false
    }
}

impl FromStr for TicketFieldRule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(?P<name>[\w ]+): (?P<ranges>\d+-\d+ (?:or \d+-\d+)?)$").unwrap();
        }

        match RE.captures(s) {
            Some(captures) => {
                let name = captures.name("name").unwrap().as_str().to_string();
                let ranges_string = captures.name("ranges").unwrap().as_str().to_string();
                let number_ranges = ranges_string
                    .split(" or ")
                    .map(number_range_string_to_range)
                    .collect::<Result<Vec<NumberRange>, ParseIntError>>()?;

                Ok(TicketFieldRule {
                    _name: name,
                    number_ranges,
                })
            }
            None => Err(anyhow::Error::msg("Could not parse ticket field rule")),
        }
    }
}

struct TicketValidator {
    ticket_field_rules: Vec<TicketFieldRule>,
}

impl TicketValidator {
    fn new(ticket_field_rules: Vec<TicketFieldRule>) -> Self {
        TicketValidator { ticket_field_rules }
    }

    fn satisfies_at_least_one_field_rule(&self, n: u64) -> bool {
        for rule in &self.ticket_field_rules {
            if rule.satisfied_by(n) {
                return true;
            }
        }

        false
    }

    fn get_invalid_field_values(&self, ticket: &Ticket) -> Vec<u64> {
        ticket
            .field_values()
            .iter()
            .copied()
            .filter(|value| !self.satisfies_at_least_one_field_rule(*value))
            .collect()
    }
}

fn number_range_string_to_range(s: &str) -> Result<NumberRange, ParseIntError> {
    let numbers: Vec<u64> = s
        .split('-')
        .map(str::parse)
        .collect::<Result<Vec<u64>, ParseIntError>>()?;

    Ok(NumberRange::new(
        *numbers.get(0).unwrap(),
        *numbers.get(1).unwrap(),
    ))
}

fn ticket_field_rules_from_input_lines(
    input_field_lines: &[String],
) -> anyhow::Result<Vec<TicketFieldRule>> {
    input_field_lines
        .iter()
        .map(|line| line.as_str().parse())
        .collect()
}

fn my_ticket_from_input_lines(input_lines: &[String]) -> anyhow::Result<Ticket> {
    Ok(ticket_from_field_values_string(
        input_lines.get(1).unwrap(),
    )?)
}

fn ticket_from_field_values_string<S: AsRef<str>>(s: S) -> Result<Ticket, ParseIntError> {
    let field_values = s
        .as_ref()
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<Vec<u64>, ParseIntError>>()?;
    Ok(Ticket::new(field_values))
}

fn nearby_tickets_from_input_lines(input_lines: &[String]) -> Result<Vec<Ticket>, ParseIntError> {
    input_lines[1..]
        .iter()
        .map(ticket_from_field_values_string)
        .collect()
}

fn parse_input_lines(
    input_lines: Vec<String>,
) -> anyhow::Result<(Vec<TicketFieldRule>, Ticket, Vec<Ticket>)> {
    let empty_line_indexes: Vec<usize> = input_lines
        .iter()
        .enumerate()
        .filter(|(_index, line)| line.is_empty())
        .map(|(index, _line)| index)
        .collect();

    let empty_line_index_1 = *empty_line_indexes.get(0).unwrap();
    let empty_line_index_2 = *empty_line_indexes.get(1).unwrap();

    let ticket_field_rules =
        ticket_field_rules_from_input_lines(&input_lines[0..empty_line_index_1])?;
    let my_ticket =
        my_ticket_from_input_lines(&input_lines[(empty_line_index_1 + 1)..empty_line_index_2])?;
    let nearby_tickets = nearby_tickets_from_input_lines(&input_lines[(empty_line_index_2 + 1)..])?;

    Ok((ticket_field_rules, my_ticket, nearby_tickets))
}

pub fn ticket_scanning_error_rate_for_input_nearby_tickets(
    input_lines: Vec<String>,
) -> anyhow::Result<u64> {
    let (ticket_field_rules, _my_ticket, nearby_tickets) = parse_input_lines(input_lines)?;
    let ticket_validator = TicketValidator::new(ticket_field_rules);

    let error_rate: u64 = nearby_tickets
        .iter()
        .map(|ticket| ticket_validator.get_invalid_field_values(ticket))
        .flatten()
        .sum();

    Ok(error_rate)
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use super::*;

    #[test]
    fn gets_ticket_scanning_error_rate_for_input() {
        let input_lines = vec![
            "class: 1-3 or 5-7",
            "row: 6-11 or 33-44",
            "seat: 13-40 or 45-50",
            "",
            "your ticket:",
            "7,1,14",
            "",
            "nearby tickets:",
            "7,3,47",
            "40,4,50",
            "55,2,20",
            "38,6,12",
        ]
        .iter()
        .map(ToString::to_string)
        .collect();

        assert_that(&ticket_scanning_error_rate_for_input_nearby_tickets(input_lines).unwrap())
            .is_equal_to(71);
    }
}
