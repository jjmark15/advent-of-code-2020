use std::collections::HashMap;
use std::num::ParseIntError;

struct RecitationGame {
    last_spoken_number: Option<u64>,
    spoken_numbers: HashMap<u64, u64>,
    starting_numbers: Vec<u64>,
    current_turn: u64,
}

impl RecitationGame {
    fn new(starting_numbers: Vec<u64>) -> Self {
        RecitationGame {
            last_spoken_number: None,
            spoken_numbers: HashMap::new(),
            starting_numbers: starting_numbers.iter().rev().copied().collect(),
            current_turn: 1,
        }
    }

    fn speak_number(&mut self, n: u64) {
        self.last_spoken_number = Some(n);
    }

    fn turns_since_n_was_spoken(&self, n: u64) -> Option<u64> {
        let last_turn = self.current_turn - 1;
        match self.spoken_numbers.get(&n) {
            Some(turn) => Some(last_turn - turn),
            None => None,
        }
    }

    fn add_spoken_number_to_memory(&mut self, spoken_number: u64) {
        self.spoken_numbers
            .insert(spoken_number, self.current_turn - 1);
    }

    fn play_next_turn(&mut self) {
        match self.starting_numbers.pop() {
            Some(n) => {
                if self.last_spoken_number.is_some() {
                    self.add_spoken_number_to_memory(self.last_spoken_number.unwrap());
                }
                self.speak_number(n);
            }
            None => {
                let turns_since_n_was_spoken =
                    self.turns_since_n_was_spoken(self.last_spoken_number.unwrap());
                self.add_spoken_number_to_memory(self.last_spoken_number.unwrap());
                self.speak_number(turns_since_n_was_spoken.unwrap_or(0))
            }
        }

        self.current_turn += 1;
    }

    fn last_spoken_number(&self) -> Option<u64> {
        self.last_spoken_number
    }
}

pub fn nth_spoken_number_in_recitation_game(
    n: u64,
    starting_numbers_string: String,
) -> anyhow::Result<u64> {
    let starting_numbers = starting_numbers_from_string(starting_numbers_string)?;
    let mut recitation_game = RecitationGame::new(starting_numbers);

    let mut loop_count = 0;
    while loop_count < n {
        recitation_game.play_next_turn();
        loop_count += 1;
    }

    Ok(recitation_game
        .last_spoken_number()
        .ok_or_else(|| anyhow::Error::msg("No words spoken"))?)
}

fn starting_numbers_from_string(
    starting_numbers_string: String,
) -> Result<Vec<u64>, ParseIntError> {
    starting_numbers_string.split(',').map(str::parse).collect()
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use super::*;

    #[test]
    fn gets_nth_spoken_number_in_recitation_game() {
        let starting_numbers = String::from("0,3,6");

        assert_that(&nth_spoken_number_in_recitation_game(9, starting_numbers).unwrap())
            .is_equal_to(4);
    }
}
