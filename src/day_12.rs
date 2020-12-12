use std::ops::{Add, AddAssign};
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

enum NavigationInstructionKind {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl FromStr for NavigationInstructionKind {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "N" => Ok(NavigationInstructionKind::North),
            "S" => Ok(NavigationInstructionKind::South),
            "E" => Ok(NavigationInstructionKind::East),
            "W" => Ok(NavigationInstructionKind::West),
            "L" => Ok(NavigationInstructionKind::Left),
            "R" => Ok(NavigationInstructionKind::Right),
            "F" => Ok(NavigationInstructionKind::Forward),
            _ => Err(anyhow::Error::msg(format!(
                "Could not parse navigation instruction kind from '{}'",
                s
            ))),
        }
    }
}

struct NavigationInstruction {
    kind: NavigationInstructionKind,
    value: u64,
}

impl NavigationInstruction {
    fn kind(&self) -> &NavigationInstructionKind {
        &self.kind
    }

    fn value(&self) -> u64 {
        self.value
    }
}

impl FromStr for NavigationInstruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(?P<kind>[A-Z])(?P<value>\d+)").unwrap();
        }

        match RE.captures(s) {
            Some(captures) => {
                let kind: NavigationInstructionKind =
                    captures.name("kind").unwrap().as_str().parse()?;
                let value: u64 = captures.name("value").unwrap().as_str().parse()?;

                Ok(NavigationInstruction { kind, value })
            }
            None => Err(anyhow::Error::msg(format!(
                "Could not parse navigation instruction from '{}'",
                s
            ))),
        }
    }
}

#[derive(Default)]
struct RelativePosition {
    east: i64,
    north: i64,
}

impl RelativePosition {
    fn east(&self) -> i64 {
        self.east
    }

    fn north(&self) -> i64 {
        self.north
    }

    fn translate_east(&mut self, value: i64) {
        self.east += value;
    }

    fn translate_north(&mut self, value: i64) {
        self.north += value;
    }
}

struct Degrees(u64);

impl Degrees {
    fn add_degree_values(lhs: i64, rhs: i64) -> u64 {
        let remainder = (lhs + rhs) % 360;

        if remainder < 0 {
            (360 + remainder) as u64
        } else {
            remainder as u64
        }
    }
}

impl Add for Degrees {
    type Output = Degrees;

    fn add(self, rhs: Self) -> Self::Output {
        Degrees(Self::add_degree_values(self.0 as i64, rhs.0 as i64))
    }
}

impl AddAssign for Degrees {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = Self::add_degree_values(self.0 as i64, rhs.0 as i64)
    }
}

impl From<i64> for Degrees {
    fn from(n: i64) -> Self {
        Degrees(Self::add_degree_values(0, n))
    }
}

struct Orientation {
    degrees: Degrees,
}

impl Orientation {
    fn new(degrees: Degrees) -> Self {
        Orientation { degrees }
    }

    fn apply_rotation(&mut self, degrees: Degrees) {
        self.degrees += degrees;
    }

    fn degrees(&self) -> &Degrees {
        &self.degrees
    }
}

struct Navigator {
    relative_position: RelativePosition,
    orientation: Orientation,
}

impl Navigator {
    fn new(starting_position: RelativePosition, starting_orientation: Orientation) -> Self {
        Navigator {
            relative_position: starting_position,
            orientation: starting_orientation,
        }
    }

    fn move_forward(&mut self, value: i64) -> anyhow::Result<()> {
        match self.orientation.degrees() {
            Degrees(0) => self.relative_position.translate_north(value),
            Degrees(90) => self.relative_position.translate_east(value),
            Degrees(180) => self.relative_position.translate_north(-value),
            Degrees(270) => self.relative_position.translate_east(-value),
            _ => {
                return Err(anyhow::Error::msg(format!(
                    "Invalid orientation '{}'",
                    self.orientation.degrees().0
                )))
            }
        }

        Ok(())
    }

    fn apply_navigation_instructions(
        &mut self,
        navigation_instructions: Vec<NavigationInstruction>,
    ) {
        for navigation_instruction in &navigation_instructions {
            match navigation_instruction.kind() {
                NavigationInstructionKind::North => self
                    .relative_position
                    .translate_north(navigation_instruction.value() as i64),
                NavigationInstructionKind::South => self
                    .relative_position
                    .translate_north(-(navigation_instruction.value as i64)),
                NavigationInstructionKind::East => self
                    .relative_position
                    .translate_east(navigation_instruction.value() as i64),
                NavigationInstructionKind::West => self
                    .relative_position
                    .translate_east(-(navigation_instruction.value() as i64)),
                NavigationInstructionKind::Left => self
                    .orientation
                    .apply_rotation(Degrees::from(-(navigation_instruction.value() as i64))),
                NavigationInstructionKind::Right => self
                    .orientation
                    .apply_rotation(Degrees(navigation_instruction.value())),
                NavigationInstructionKind::Forward => self
                    .move_forward(navigation_instruction.value() as i64)
                    .unwrap(),
            }
        }
    }

    fn relative_position(&self) -> &RelativePosition {
        &self.relative_position
    }
}

fn manhattan_distance(position: &RelativePosition) -> u64 {
    (position.east().abs() + position.north().abs()) as u64
}

pub fn get_manhattan_distance_to_directed_location(
    navigation_instruction_strings: Vec<String>,
) -> anyhow::Result<u64> {
    let navigation_instructions = navigation_instruction_strings
        .iter()
        .map(|s| s.parse())
        .collect::<anyhow::Result<Vec<NavigationInstruction>>>()?;

    let mut navigator = Navigator::new(RelativePosition::default(), Orientation::new(90.into()));

    navigator.apply_navigation_instructions(navigation_instructions);
    Ok(manhattan_distance(navigator.relative_position()))
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use super::*;

    #[test]
    fn gets_manhattan_distance_to_directed_location() {
        let navigation_instruction_strings = vec!["F10", "N3", "F7", "R90", "F11"]
            .iter()
            .map(ToString::to_string)
            .collect();

        assert_that(
            &get_manhattan_distance_to_directed_location(navigation_instruction_strings).unwrap(),
        )
        .is_equal_to(25);
    }
}
