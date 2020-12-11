use std::str::FromStr;

#[derive(Eq, PartialEq)]
enum SeatOccupancy {
    Occupied,
    Empty,
}

#[derive(Eq, PartialEq)]
enum SeatingElement {
    Floor,
    Seat(SeatOccupancy),
}

impl FromStr for SeatingElement {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(SeatingElement::Seat(SeatOccupancy::Empty)),
            "." => Ok(SeatingElement::Floor),
            "#" => Ok(SeatingElement::Seat(SeatOccupancy::Occupied)),
            _ => Err(anyhow::Error::msg(format!(
                "Could not parse seating element '{}'",
                s
            ))),
        }
    }
}

struct Direction {
    vertical_component: VerticalDirectionComponent,
    horizontal_component: HorizontalDirectionComponent,
}

impl Direction {
    fn new(
        vertical_component: VerticalDirectionComponent,
        horizontal_component: HorizontalDirectionComponent,
    ) -> Self {
        Direction {
            vertical_component,
            horizontal_component,
        }
    }

    fn vertical_component(&self) -> &VerticalDirectionComponent {
        &self.vertical_component
    }

    fn horizontal_component(&self) -> &HorizontalDirectionComponent {
        &self.horizontal_component
    }
}

enum VerticalDirectionComponent {
    Above,
    Equal,
    Below,
}

enum HorizontalDirectionComponent {
    Right,
    Equal,
    Left,
}

#[derive(Copy, Clone)]
struct SeatingPosition {
    row: usize,
    column: usize,
}

impl SeatingPosition {
    fn new(row: usize, column: usize) -> Self {
        SeatingPosition { row, column }
    }

    fn get_relative_position(&self, direction: &Direction) -> Option<Self> {
        let row: i64 = match direction.vertical_component() {
            VerticalDirectionComponent::Above => self.row as i64 + 1,
            VerticalDirectionComponent::Equal => self.row as i64,
            VerticalDirectionComponent::Below => self.row as i64 - 1,
        };

        let column: i64 = match direction.horizontal_component() {
            HorizontalDirectionComponent::Right => self.column as i64 + 1,
            HorizontalDirectionComponent::Equal => self.column as i64,
            HorizontalDirectionComponent::Left => self.column as i64 - 1,
        };

        if row < 0 || column < 0 {
            return None;
        }

        Some(SeatingPosition::new(row as usize, column as usize))
    }

    fn row(&self) -> usize {
        self.row
    }

    fn column(&self) -> usize {
        self.column
    }

    fn adjacent_positions(&self) -> Vec<Self> {
        [
            Direction::new(
                VerticalDirectionComponent::Above,
                HorizontalDirectionComponent::Equal,
            ),
            Direction::new(
                VerticalDirectionComponent::Above,
                HorizontalDirectionComponent::Right,
            ),
            Direction::new(
                VerticalDirectionComponent::Above,
                HorizontalDirectionComponent::Left,
            ),
            Direction::new(
                VerticalDirectionComponent::Below,
                HorizontalDirectionComponent::Equal,
            ),
            Direction::new(
                VerticalDirectionComponent::Below,
                HorizontalDirectionComponent::Right,
            ),
            Direction::new(
                VerticalDirectionComponent::Below,
                HorizontalDirectionComponent::Left,
            ),
            Direction::new(
                VerticalDirectionComponent::Equal,
                HorizontalDirectionComponent::Right,
            ),
            Direction::new(
                VerticalDirectionComponent::Equal,
                HorizontalDirectionComponent::Left,
            ),
        ]
        .iter()
        .map(|direction| self.get_relative_position(direction))
        .filter(Option::is_some)
        .map(|option_position: Option<Self>| option_position.unwrap())
        .collect::<Vec<Self>>()
    }
}

struct SeatingState {
    seating_positions: Vec<Vec<SeatingElement>>,
    reached_stability: bool,
}

impl SeatingState {
    fn new(seating_positions: Vec<Vec<SeatingElement>>) -> Self {
        SeatingState {
            seating_positions,
            reached_stability: false,
        }
    }

    fn valid_adjacent_positions(&self, position: &SeatingPosition) -> Vec<SeatingPosition> {
        position
            .adjacent_positions()
            .iter()
            .filter(|adjacent_position| {
                adjacent_position.row() < self.seating_positions.len()
                    && adjacent_position.column() < self.seating_positions.first().unwrap().len()
            })
            .copied()
            .collect::<Vec<SeatingPosition>>()
    }

    fn count_adjacent_occupied_seats(&self, seating_position: &SeatingPosition) -> usize {
        self.valid_adjacent_positions(seating_position)
            .iter()
            .filter(|position| {
                matches!(
                    self.get_seating_element_at_position(position).unwrap(),
                    SeatingElement::Seat(SeatOccupancy::Occupied)
                )
            })
            .count()
    }

    fn get_seating_element_at_position(
        &self,
        seating_position: &SeatingPosition,
    ) -> Option<&SeatingElement> {
        match self.seating_positions.get(seating_position.row()) {
            Some(row) => row.get(seating_position.column()),
            None => None,
        }
    }

    fn get_new_seating_element_at_position(
        &self,
        seating_position: SeatingPosition,
    ) -> SeatingElement {
        match self
            .get_seating_element_at_position(&seating_position)
            .unwrap()
        {
            SeatingElement::Seat(SeatOccupancy::Occupied) => {
                if self.count_adjacent_occupied_seats(&seating_position) >= 4 {
                    SeatingElement::Seat(SeatOccupancy::Empty)
                } else {
                    SeatingElement::Seat(SeatOccupancy::Occupied)
                }
            }
            SeatingElement::Seat(SeatOccupancy::Empty) => {
                if self.count_adjacent_occupied_seats(&seating_position) == 0 {
                    SeatingElement::Seat(SeatOccupancy::Occupied)
                } else {
                    SeatingElement::Seat(SeatOccupancy::Empty)
                }
            }
            SeatingElement::Floor => SeatingElement::Floor,
        }
    }

    fn apply_occupancy_rule(&mut self) {
        self.reached_stability = true;
        let mut element_changes = 0;

        self.seating_positions = self
            .seating_positions
            .iter()
            .enumerate()
            .map(|(row_index, seating_row)| {
                seating_row
                    .iter()
                    .enumerate()
                    .map(|(column_index, seating_element)| {
                        let seating_position = SeatingPosition::new(row_index, column_index);
                        let new_seating_element =
                            self.get_new_seating_element_at_position(seating_position);
                        if &new_seating_element != seating_element {
                            element_changes += 1;
                        }
                        new_seating_element
                    })
                    .collect::<Vec<SeatingElement>>()
            })
            .collect::<Vec<Vec<SeatingElement>>>();

        if element_changes > 0 {
            self.register_unstable();
        }
    }

    fn register_unstable(&mut self) {
        if self.reached_stability {
            self.reached_stability = false;
        }
    }

    fn is_stable(&self) -> bool {
        self.reached_stability
    }

    fn occupied_seats(&self) -> usize {
        self.seating_positions
            .iter()
            .flatten()
            .filter(|element| matches!(element, SeatingElement::Seat(SeatOccupancy::Occupied)))
            .count()
    }
}

fn seating_state_from_element_row_strings(
    seating_element_row_strings: Vec<String>,
) -> anyhow::Result<SeatingState> {
    let seating_positions = seating_element_row_strings
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| c.to_string().as_str().parse())
                .collect::<anyhow::Result<Vec<SeatingElement>>>()
        })
        .collect::<anyhow::Result<Vec<Vec<SeatingElement>>>>();

    Ok(SeatingState::new(seating_positions?))
}

pub fn count_occupied_seats_after_occupancy_stabilisation(
    seating_element_row_strings: Vec<String>,
) -> anyhow::Result<usize> {
    let mut seating_state = seating_state_from_element_row_strings(seating_element_row_strings)?;

    while !seating_state.is_stable() {
        seating_state.apply_occupancy_rule();
    }

    Ok(seating_state.occupied_seats())
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use super::*;

    #[test]
    fn counts_occupied_seats_after_occupancy_stabilisation() {
        let seating_element_row_strings: Vec<String> = vec![
            "L.LL.LL.LL",
            "LLLLLLL.LL",
            "L.L.L..L..",
            "LLLL.LL.LL",
            "L.LL.LL.LL",
            "L.LLLLL.LL",
            "..L.L.....",
            "LLLLLLLLLL",
            "L.LLLLLL.L",
            "L.LLLLL.LL",
        ]
        .iter()
        .map(ToString::to_string)
        .collect();

        assert_that(
            &count_occupied_seats_after_occupancy_stabilisation(seating_element_row_strings)
                .unwrap(),
        )
        .is_equal_to(37);
    }
}
