use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

struct PlaneSpecification {
    rows: u32,
    columns: u32,
}

impl PlaneSpecification {
    fn new(rows: u32, columns: u32) -> Self {
        PlaneSpecification { rows, columns }
    }

    fn lowest_row(&self) -> u32 {
        0
    }

    fn highest_row(&self) -> u32 {
        self.rows - 1
    }

    fn lowest_column(&self) -> u32 {
        0
    }

    fn highest_column(&self) -> u32 {
        self.columns - 1
    }
}

#[cfg_attr(test, derive(Debug))]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct SeatId {
    value: u32,
}

impl SeatId {
    fn from_seat_position(seat_position: SeatPosition) -> Self {
        SeatId {
            value: seat_position.row() * 8 + seat_position.column(),
        }
    }

    fn value(&self) -> u32 {
        self.value
    }
}

struct SeatCode {
    row_segments: Vec<SeatCodeRowSegment>,
    column_segments: Vec<SeatCodeColumnSegment>,
}

impl SeatCode {
    fn row_segments(&self) -> &Vec<SeatCodeRowSegment> {
        &self.row_segments
    }

    fn column_segments(&self) -> &Vec<SeatCodeColumnSegment> {
        &self.column_segments
    }
}

impl FromStr for SeatCode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^([FB]{7})([RL]{3})$").unwrap();
        }

        match RE.captures(s) {
            Some(captures) => Ok(SeatCode {
                row_segments: captures
                    .get(1)
                    .unwrap()
                    .as_str()
                    .chars()
                    .map(|c| c.to_string().as_str().parse())
                    .collect::<anyhow::Result<Vec<SeatCodeRowSegment>>>()?,
                column_segments: captures
                    .get(2)
                    .unwrap()
                    .as_str()
                    .chars()
                    .map(|c| c.to_string().as_str().parse())
                    .collect::<anyhow::Result<Vec<SeatCodeColumnSegment>>>()?,
            }),
            None => Err(anyhow::Error::msg("Could not parse seat code")),
        }
    }
}

#[derive(Copy, Clone)]
enum SeatCodeRowSegment {
    Front,
    Back,
}

impl FromStr for SeatCodeRowSegment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "F" => Ok(SeatCodeRowSegment::Front),
            "B" => Ok(SeatCodeRowSegment::Back),
            _ => Err(anyhow::Error::msg("Invalid seat code row segment")),
        }
    }
}

#[derive(Copy, Clone)]
enum SeatCodeColumnSegment {
    Left,
    Right,
}

impl FromStr for SeatCodeColumnSegment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(SeatCodeColumnSegment::Left),
            "R" => Ok(SeatCodeColumnSegment::Right),
            _ => Err(anyhow::Error::msg("Invalid seat code column segment")),
        }
    }
}

#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
struct SeatPosition {
    row: u32,
    column: u32,
}

impl SeatPosition {
    fn new(row: u32, column: u32) -> Self {
        SeatPosition { row, column }
    }

    fn row(&self) -> u32 {
        self.row
    }

    fn column(&self) -> u32 {
        self.column
    }
}

struct SeatFinder {
    plane_specification: PlaneSpecification,
}

impl SeatFinder {
    fn new(plane_specification: PlaneSpecification) -> Self {
        SeatFinder {
            plane_specification,
        }
    }

    fn find_seat_row(&self, seat_code_segments: Vec<SeatCodeRowSegment>) -> u32 {
        let mut range = BinarySpacePartitionRange::new(
            self.plane_specification.highest_row(),
            self.plane_specification.lowest_row(),
        );

        seat_code_segments.iter().for_each(|segment| match segment {
            SeatCodeRowSegment::Front => range.use_lower(),
            SeatCodeRowSegment::Back => range.use_higher(),
        });

        range.single_result().unwrap()
    }

    fn find_seat_column(&self, seat_code_segments: Vec<SeatCodeColumnSegment>) -> u32 {
        let mut range = BinarySpacePartitionRange::new(
            self.plane_specification.highest_column(),
            self.plane_specification.lowest_column(),
        );

        seat_code_segments.iter().for_each(|segment| match segment {
            SeatCodeColumnSegment::Left => range.use_lower(),
            SeatCodeColumnSegment::Right => range.use_higher(),
        });

        range.single_result().unwrap()
    }

    fn find_seat(&self, seat_code: &SeatCode) -> SeatPosition {
        let row = self.find_seat_row(seat_code.row_segments().clone());
        let column = self.find_seat_column(seat_code.column_segments().clone());
        SeatPosition::new(row, column)
    }
}

struct BinarySpacePartitionRange {
    upper: u32,
    lower: u32,
}

impl BinarySpacePartitionRange {
    fn new(upper: u32, lower: u32) -> Self {
        BinarySpacePartitionRange { upper, lower }
    }

    fn use_lower(&mut self) {
        let gap_from_lower: f32 = ((self.upper as f32 - self.lower as f32) / 2_f32).floor();
        self.upper = self.lower + gap_from_lower as u32
    }

    fn use_higher(&mut self) {
        let gap_from_lower: f32 = ((self.upper as f32 - self.lower as f32) / 2_f32).ceil();
        self.lower += gap_from_lower as u32
    }

    fn single_result(&self) -> Option<u32> {
        if self.lower == self.upper {
            Some(self.lower)
        } else {
            None
        }
    }
}

fn seat_ids(seat_code_strings: Vec<String>) -> anyhow::Result<Vec<SeatId>> {
    let seat_codes = seat_code_strings
        .iter()
        .map(|s| s.as_str().parse())
        .collect::<anyhow::Result<Vec<SeatCode>>>()?;

    let seat_finder = SeatFinder::new(PlaneSpecification::new(128, 8));

    Ok(seat_codes
        .iter()
        .map(|seat_code| seat_finder.find_seat(seat_code))
        .map(SeatId::from_seat_position)
        .collect())
}

pub fn highest_seat_id_on_plane(seat_code_strings: Vec<String>) -> anyhow::Result<u32> {
    Ok(seat_ids(seat_code_strings)?
        .iter()
        .max()
        .ok_or_else(|| anyhow::Error::msg("Empty list of seat codes"))?
        .value())
}

pub fn find_my_empty_seat_id(seat_code_strings: Vec<String>) -> anyhow::Result<u32> {
    let mut sorted_seat_ids = seat_ids(seat_code_strings)?;
    sorted_seat_ids.sort();

    for (i, seat_id) in sorted_seat_ids
        .get(1..sorted_seat_ids.len() - 2)
        .unwrap()
        .iter()
        .enumerate()
    {
        if seat_id.value() - 1 != sorted_seat_ids.get(i).unwrap().value() {
            return Ok(seat_id.value() - 1);
        }
    }

    Err(anyhow::Error::msg("Did not find my seat"))
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use super::*;

    #[test]
    fn calculates_seat_id_from_seat_position() {
        let seat_position = SeatPosition::new(44, 5);

        assert_that(&SeatId::from_seat_position(seat_position).value()).is_equal_to(357)
    }

    #[test]
    fn finds_seat_from_seat_code() {
        let plane_specification = PlaneSpecification::new(128, 8);
        let seat_finder = SeatFinder::new(plane_specification);

        assert_that(&seat_finder.find_seat(&SeatCode::from_str("BFFFBBFRRR").unwrap()))
            .is_equal_to(SeatPosition::new(70, 7));
    }
}
