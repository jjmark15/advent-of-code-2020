use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::num::ParseIntError;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;
use std::hash::Hash;

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

#[derive(Clone)]
struct UnidentifiedTicketFieldValues {
    field_values: Vec<u64>,
}

impl UnidentifiedTicketFieldValues {
    fn new(field_values: Vec<u64>) -> Self {
        UnidentifiedTicketFieldValues { field_values }
    }

    fn field_values(&self) -> &Vec<u64> {
        &self.field_values
    }
}

struct IdentifiedTicketFieldValues {
    map: HashMap<String, u64>,
}

impl IdentifiedTicketFieldValues {
    fn new(map: HashMap<String, u64>) -> Self {
        IdentifiedTicketFieldValues { map }
    }

    fn values_of_fields_starting_with(&self, name_start: &str) -> Vec<u64> {
        self.map
            .keys()
            .filter(|name| name.starts_with(name_start))
            .map(|name| *self.map.get(name).unwrap())
            .collect()
    }
}

struct TicketFieldRule {
    name: String,
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

    fn name(&self) -> &str {
        self.name.as_str()
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
                    name: name,
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

    fn valid(&self, ticket: &UnidentifiedTicketFieldValues) -> bool {
        for field_value in ticket.field_values() {
            if !self.satisfies_at_least_one_field_rule(*field_value) {
                return false;
            }
        }

        true
    }

    fn get_invalid_field_values(&self, ticket: &UnidentifiedTicketFieldValues) -> Vec<u64> {
        ticket
            .field_values()
            .iter()
            .copied()
            .filter(|value| !self.satisfies_at_least_one_field_rule(*value))
            .collect()
    }

    fn valid_tickets(
        &self,
        ticket_values: Vec<UnidentifiedTicketFieldValues>,
    ) -> Vec<UnidentifiedTicketFieldValues> {
        ticket_values
            .iter()
            .cloned()
            .filter(|ticket_value| self.valid(ticket_value))
            .collect()
    }

    fn new_possible_field_names_map(&self) -> HashMap<usize, HashSet<String>> {
        let field_names: Vec<String> = self
            .ticket_field_rules
            .iter()
            .map(|rule| rule.name().to_string())
            .collect();

        HashMap::from_iter(
            (0..field_names.len()).map(|index| (index, HashSet::from_iter(field_names.clone()))),
        )
    }

    fn identify_ticket_field_indexes(
        &self,
        ticket_values: Vec<UnidentifiedTicketFieldValues>,
    ) -> HashMap<usize, String> {
        let mut possible_field_names: HashMap<usize, HashSet<String>> =
            self.new_possible_field_names_map();
        let mut identified_field_indexes: HashMap<usize, String> = HashMap::new();
        let valid_tickets = self.valid_tickets(ticket_values);
        let indexes: Vec<usize> = possible_field_names.keys().cloned().collect();

        valid_tickets.iter().for_each(|ticket| {
            ticket
                .field_values()
                .iter()
                .enumerate()
                .for_each(|(field_index, value)| {
                    self.ticket_field_rules.iter().for_each(|rule| {
                        if !rule.satisfied_by(*value) {
                            possible_field_names
                                .get_mut(&field_index)
                                .unwrap()
                                .remove(rule.name());
                        }
                    })
                });
        });

        while identified_field_indexes.keys().len() < indexes.len() {
            for field_index in &indexes {
                if possible_field_names.get(&field_index).unwrap().len() == 1 {
                    let field_name =
                        only_element_in_set(possible_field_names.get(&field_index).unwrap());
                    identified_field_indexes.insert(*field_index, field_name.clone());

                    possible_field_names
                        .values_mut()
                        .for_each(|possible_index_field_names| {
                            possible_index_field_names.remove(field_name.as_str());
                        });
                }
            }
        }

        identified_field_indexes
    }

    fn identify_ticket_values_from_nearby_tickets(
        &self,
        ticket: UnidentifiedTicketFieldValues,
        nearby_ticket_values: Vec<UnidentifiedTicketFieldValues>,
    ) -> IdentifiedTicketFieldValues {
        let identified_ticket_indexes = self.identify_ticket_field_indexes(nearby_ticket_values);
        let identified_ticket_map: HashMap<String, u64> = identified_ticket_indexes
            .keys()
            .map(|key| {
                (
                    identified_ticket_indexes.get(key).unwrap().to_string(),
                    *ticket.field_values().get(*key).unwrap(),
                )
            })
            .collect();

        IdentifiedTicketFieldValues::new(identified_ticket_map)
    }
}

fn only_element_in_set<T: Hash + Default + Clone>(set: &HashSet<T>) -> T {
    set.iter()
        .fold(T::default(), |_acc, value| value.to_owned())
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

fn my_ticket_from_input_lines(
    input_lines: &[String],
) -> anyhow::Result<UnidentifiedTicketFieldValues> {
    Ok(ticket_from_field_values_string(
        input_lines.get(1).unwrap(),
    )?)
}

fn ticket_from_field_values_string<S: AsRef<str>>(
    s: S,
) -> Result<UnidentifiedTicketFieldValues, ParseIntError> {
    let field_values = s
        .as_ref()
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<Vec<u64>, ParseIntError>>()?;
    Ok(UnidentifiedTicketFieldValues::new(field_values))
}

fn nearby_tickets_from_input_lines(
    input_lines: &[String],
) -> Result<Vec<UnidentifiedTicketFieldValues>, ParseIntError> {
    input_lines[1..]
        .iter()
        .map(ticket_from_field_values_string)
        .collect()
}

fn parse_input_lines(
    input_lines: Vec<String>,
) -> anyhow::Result<(
    Vec<TicketFieldRule>,
    UnidentifiedTicketFieldValues,
    Vec<UnidentifiedTicketFieldValues>,
)> {
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

pub fn product_of_my_departure_field_values(input_lines: Vec<String>) -> anyhow::Result<u64> {
    let (ticket_field_rules, my_ticket, nearby_tickets) = parse_input_lines(input_lines)?;
    let ticket_validator = TicketValidator::new(ticket_field_rules);

    let identified_ticket_values =
        ticket_validator.identify_ticket_values_from_nearby_tickets(my_ticket, nearby_tickets);

    Ok(identified_ticket_values
        .values_of_fields_starting_with("departure")
        .iter()
        .product())
}

#[cfg(test)]
mod tests {
    use std::iter::FromIterator;

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

    #[test]
    fn identifies_fields_in_my_ticket() {
        let input_lines = vec![
            "class: 0-1 or 4-19",
            "row: 0-5 or 8-19",
            "seat: 0-13 or 16-19",
            "",
            "your ticket:",
            "11,12,13",
            "",
            "nearby tickets:",
            "3,9,18",
            "15,1,5",
            "5,14,9",
        ]
        .iter()
        .map(ToString::to_string)
        .collect();
        let (ticket_field_rules, _my_ticket, nearby_tickets) =
            parse_input_lines(input_lines).unwrap();
        let ticket_validator = TicketValidator::new(ticket_field_rules);

        assert_that(&ticket_validator.identify_ticket_field_indexes(nearby_tickets)).is_equal_to(
            HashMap::from_iter(vec![
                (0, "row".to_string()),
                (1, "class".to_string()),
                (2, "seat".to_string()),
            ]),
        );
    }
}
