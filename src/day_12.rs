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

#[derive(Default, Copy, Clone)]
struct RelativePosition {
    east: i64,
    north: i64,
}

impl RelativePosition {
    fn new(east: i64, north: i64) -> Self {
        RelativePosition { east, north }
    }

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

    fn set_east(&mut self, east: i64) {
        self.east = east;
    }

    fn set_north(&mut self, north: i64) {
        self.north = north;
    }
}

impl AddAssign for RelativePosition {
    fn add_assign(&mut self, rhs: Self) {
        self.north += rhs.north;
        self.east += rhs.east;
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

struct ShipNavigator {
    relative_position: RelativePosition,
    orientation: Orientation,
}

impl ShipNavigator {
    fn new(starting_position: RelativePosition, starting_orientation: Orientation) -> Self {
        ShipNavigator {
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

struct ShipWaypointNavigator {
    ship_position: RelativePosition,
    waypoint_position_relative_to_ship: RelativePosition,
}

impl ShipWaypointNavigator {
    fn new(
        ship_starting_position: RelativePosition,
        waypoint_position_relative_to_ship: RelativePosition,
    ) -> Self {
        ShipWaypointNavigator {
            ship_position: ship_starting_position,
            waypoint_position_relative_to_ship,
        }
    }

    fn move_ship_to_waypoint_times(&mut self, value: u64) {
        (0..value).for_each(|_| self.ship_position += self.waypoint_position_relative_to_ship);
    }

    fn rotate_waypoint_about_ship_by_90_degrees(&mut self, times: u64) {
        (0..times).for_each(|_| {
            let new_east = self.waypoint_position_relative_to_ship.north;
            let new_north = -self.waypoint_position_relative_to_ship.east;
            self.waypoint_position_relative_to_ship.set_east(new_east);
            self.waypoint_position_relative_to_ship.set_north(new_north);
        });
    }

    fn rotate_waypoint_about_ship(&mut self, degrees: Degrees) -> anyhow::Result<()> {
        match degrees {
            Degrees(0) => (),
            Degrees(90) => self.rotate_waypoint_about_ship_by_90_degrees(1),
            Degrees(180) => self.rotate_waypoint_about_ship_by_90_degrees(2),
            Degrees(270) => self.rotate_waypoint_about_ship_by_90_degrees(3),
            _ => {
                return Err(anyhow::Error::msg(format!(
                    "Cannot rotate about {}",
                    degrees.0
                )))
            }
        };

        Ok(())
    }

    fn apply_navigation_instructions(
        &mut self,
        navigation_instructions: Vec<NavigationInstruction>,
    ) {
        for navigation_instruction in &navigation_instructions {
            match navigation_instruction.kind() {
                NavigationInstructionKind::North => self
                    .waypoint_position_relative_to_ship
                    .translate_north(navigation_instruction.value() as i64),
                NavigationInstructionKind::South => self
                    .waypoint_position_relative_to_ship
                    .translate_north(-(navigation_instruction.value as i64)),
                NavigationInstructionKind::East => self
                    .waypoint_position_relative_to_ship
                    .translate_east(navigation_instruction.value() as i64),
                NavigationInstructionKind::West => self
                    .waypoint_position_relative_to_ship
                    .translate_east(-(navigation_instruction.value() as i64)),
                NavigationInstructionKind::Left => self
                    .rotate_waypoint_about_ship(Degrees::from(
                        -(navigation_instruction.value() as i64),
                    ))
                    .unwrap(),
                NavigationInstructionKind::Right => self
                    .rotate_waypoint_about_ship(Degrees(navigation_instruction.value()))
                    .unwrap(),
                NavigationInstructionKind::Forward => {
                    self.move_ship_to_waypoint_times(navigation_instruction.value());
                }
            }
        }
    }

    fn ship_position(&self) -> &RelativePosition {
        &self.ship_position
    }
}

fn manhattan_distance(position: &RelativePosition) -> u64 {
    (position.east().abs() + position.north().abs()) as u64
}

fn navigation_instructions_from_strings(
    navigation_instruction_strings: Vec<String>,
) -> anyhow::Result<Vec<NavigationInstruction>> {
    navigation_instruction_strings
        .iter()
        .map(|s| s.parse())
        .collect::<anyhow::Result<Vec<NavigationInstruction>>>()
}

pub fn manhattan_distance_to_directed_location(
    navigation_instruction_strings: Vec<String>,
) -> anyhow::Result<u64> {
    let mut navigator =
        ShipNavigator::new(RelativePosition::default(), Orientation::new(90.into()));

    navigator.apply_navigation_instructions(navigation_instructions_from_strings(
        navigation_instruction_strings,
    )?);
    Ok(manhattan_distance(navigator.relative_position()))
}

pub fn manhattan_distance_to_directed_location_with_waypoint_navigation(
    navigation_instruction_strings: Vec<String>,
) -> anyhow::Result<u64> {
    let mut navigator =
        ShipWaypointNavigator::new(RelativePosition::default(), RelativePosition::new(10, 1));

    navigator.apply_navigation_instructions(navigation_instructions_from_strings(
        navigation_instruction_strings,
    )?);
    Ok(manhattan_distance(navigator.ship_position()))
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
            &manhattan_distance_to_directed_location(navigation_instruction_strings).unwrap(),
        )
        .is_equal_to(25);
    }

    #[test]
    fn gets_manhattan_distance_to_directed_location_with_waypoint_navigation() {
        let navigation_instruction_strings = vec!["F10", "N3", "F7", "R90", "F11"]
            .iter()
            .map(ToString::to_string)
            .collect();

        assert_that(
            &manhattan_distance_to_directed_location_with_waypoint_navigation(
                navigation_instruction_strings,
            )
            .unwrap(),
        )
        .is_equal_to(286);
    }
}
