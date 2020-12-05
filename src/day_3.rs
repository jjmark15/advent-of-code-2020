use std::str::FromStr;

#[derive(Copy, Clone)]
pub enum MovementDirection {
    Right,
    Down,
}

pub enum TravelMapElement {
    Tree,
    Open,
}

impl FromStr for TravelMapElement {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "#" {
            Ok(TravelMapElement::Tree)
        } else if s == "." {
            Ok(TravelMapElement::Open)
        } else {
            Err(anyhow::Error::msg("Could not parse map element"))
        }
    }
}

#[derive(Copy, Clone)]
pub struct TravelMapPosition {
    x: usize,
    y: usize,
}

impl TravelMapPosition {
    pub fn new(x: usize, y: usize) -> Self {
        TravelMapPosition { x, y }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }
}

pub struct TravelMapSegment {
    map_rows: Vec<String>,
}

impl TravelMapSegment {
    pub fn new(map_rows: Vec<String>) -> Self {
        TravelMapSegment { map_rows }
    }

    pub fn element_at_position(
        &self,
        position: TravelMapPosition,
    ) -> anyhow::Result<TravelMapElement> {
        self.map_rows
            .get(position.y())
            .ok_or_else(|| anyhow::Error::msg("Tried to access position out of map bounds"))?
            .chars()
            .nth(position.x())
            .ok_or_else(|| anyhow::Error::msg("Tried to access position out of map bounds"))?
            .to_string()
            .as_str()
            .parse()
    }

    pub fn height(&self) -> usize {
        self.map_rows.len()
    }

    pub fn width(&self) -> usize {
        self.map_rows.get(0).unwrap().len()
    }
}

pub struct TravelMapReader {
    segment: TravelMapSegment,
}

impl TravelMapReader {
    pub fn new(segment: TravelMapSegment) -> Self {
        TravelMapReader { segment }
    }

    pub fn element_at_position(
        &self,
        position: TravelMapPosition,
    ) -> anyhow::Result<TravelMapElement> {
        let translated_position =
            TravelMapPosition::new(position.x() % self.segment.width(), position.y());
        self.segment.element_at_position(translated_position)
    }

    pub fn position_with_move_applied(
        &self,
        position: TravelMapPosition,
        movement_direction: MovementDirection,
    ) -> TravelMapPosition {
        match movement_direction {
            MovementDirection::Right => TravelMapPosition::new(position.x() + 1, position.y()),
            MovementDirection::Down => TravelMapPosition::new(position.x(), position.y() + 1),
        }
    }

    pub fn position_is_below_map(&self, position: TravelMapPosition) -> bool {
        position.y() >= self.map_height()
    }

    pub fn map_height(&self) -> usize {
        self.segment.height()
    }
}

pub struct TobogganRideState {
    current_position: TravelMapPosition,
    map_reader: TravelMapReader,
}

impl TobogganRideState {
    pub fn new(starting_position: TravelMapPosition, map_reader: TravelMapReader) -> Self {
        TobogganRideState {
            current_position: starting_position,
            map_reader,
        }
    }

    pub fn travel(&mut self, direction: MovementDirection) -> anyhow::Result<()> {
        let new_position = self
            .map_reader
            .position_with_move_applied(self.current_position, direction);

        if self.map_reader.position_is_below_map(new_position) {
            return Err(anyhow::Error::msg("Position is below the map"));
        }
        self.current_position = new_position;
        Ok(())
    }

    pub fn at_end_of_map(&self) -> bool {
        self.current_position.y() + 1 == self.map_reader.map_height()
    }

    pub fn element_at_current_position(&self) -> anyhow::Result<TravelMapElement> {
        self.map_reader.element_at_position(self.current_position)
    }
}

pub fn count_encountered_trees_for_movement_sequence(
    map_rows: Vec<String>,
    movement_sequence: Vec<MovementDirection>,
) -> u64 {
    let map_segment = TravelMapSegment::new(map_rows);
    let mut ride_state = TobogganRideState::new(
        TravelMapPosition::new(0, 0),
        TravelMapReader::new(map_segment),
    );
    let mut tree_count = 0;

    while !ride_state.at_end_of_map() {
        for &direction in &movement_sequence {
            if ride_state.travel(direction).is_err() {
                break;
            }
        }

        match ride_state.element_at_current_position() {
            Ok(TravelMapElement::Tree) => tree_count += 1,
            Ok(TravelMapElement::Open) => (),
            _ => break,
        }
    }
    tree_count
}

pub fn product_of_tree_encounters_for_movement_sequences(
    map_rows: Vec<String>,
    movement_sequences: Vec<Vec<MovementDirection>>,
) -> u64 {
    movement_sequences
        .iter()
        .map(|movement_sequence| {
            count_encountered_trees_for_movement_sequence(
                map_rows.clone(),
                movement_sequence.clone(),
            )
        })
        .product()
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use crate::day_3::MovementDirection::{Down, Right};

    use super::*;

    #[test]
    fn counts_hit_trees_along_a_repeated_movement_sequence() {
        let map_rows = vec![
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#",
        ]
        .iter()
        .map(ToString::to_string)
        .collect();

        assert_that(&count_encountered_trees_for_movement_sequence(
            map_rows,
            vec![Right, Right, Right, Down],
        ))
        .is_equal_to(7)
    }

    #[test]
    fn calculates_product_of_hit_trees_along_repeated_movement_sequences() {
        let map_rows = vec![
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#",
        ]
        .iter()
        .map(ToString::to_string)
        .collect();
        let movement_sequences = vec![
            vec![Right, Down],
            vec![Right, Right, Right, Down],
            vec![Right, Right, Right, Right, Right, Down],
            vec![Right, Right, Right, Right, Right, Right, Right, Down],
            vec![Right, Down, Down],
        ];

        assert_that(&product_of_tree_encounters_for_movement_sequences(
            map_rows,
            movement_sequences,
        ))
        .is_equal_to(2 * 7 * 3 * 4 * 2)
    }
}
