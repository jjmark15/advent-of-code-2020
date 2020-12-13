use std::cmp::{max, min, Ordering};
use std::num::ParseIntError;

struct XMASDecrypter;

impl XMASDecrypter {
    fn new() -> Self {
        XMASDecrypter
    }

    fn contains_pair_that_sums_to(&self, xmas_encrypted_messages: &[u64], target: &u64) -> bool {
        for (outer_index, a) in xmas_encrypted_messages.iter().enumerate() {
            for (inner_index, b) in xmas_encrypted_messages.iter().enumerate() {
                if outer_index != inner_index && a + b == *target {
                    return true;
                }
            }
        }
        false
    }

    fn find_first_encoding_error_instance(
        &self,
        xmas_encrypted_messages: &[u64],
        preamble_length: usize,
    ) -> anyhow::Result<u64> {
        let result = xmas_encrypted_messages
            .iter()
            .enumerate()
            .skip_while(|(index, _n)| index < &preamble_length)
            .try_for_each(|(index, n)| {
                if self.contains_pair_that_sums_to(
                    &xmas_encrypted_messages[(index - preamble_length)..index],
                    n,
                ) {
                    Ok(())
                } else {
                    Err(n)
                }
            });

        match result {
            Ok(_) => Err(anyhow::Error::msg("Did not find weakness")),
            Err(e) => Ok(*e),
        }
    }

    fn get_encryption_weakness(
        &self,
        xmas_encrypted_messages: &[u64],
        preamble_length: usize,
    ) -> anyhow::Result<u64> {
        let first_encoding_error =
            self.find_first_encoding_error_instance(xmas_encrypted_messages, preamble_length)?;

        for (outer_index, first) in xmas_encrypted_messages.iter().enumerate() {
            let mut accumulator = 0;
            let mut smallest = *first;
            let mut largest = *first;

            for (_inner_index, n) in xmas_encrypted_messages
                .iter()
                .enumerate()
                .skip_while(|(i, _n)| i < &outer_index)
            {
                accumulator += n;
                smallest = min(smallest, *n);
                largest = max(largest, *n);

                match accumulator.cmp(&first_encoding_error) {
                    Ordering::Equal => return Ok(smallest + largest),
                    Ordering::Greater => break,
                    Ordering::Less => (),
                }
            }
        }

        Err(anyhow::Error::msg("Could not find encryption weakness"))
    }
}

pub fn first_xmas_encoding_error(
    xmas_encrypted_message_strings: Vec<String>,
    preamble_length: usize,
) -> anyhow::Result<u64> {
    let xmas_encrypted_messages: Vec<u64> = xmas_encrypted_message_strings
        .iter()
        .map(|s| s.parse())
        .collect::<Result<Vec<u64>, ParseIntError>>()?;
    let xmas_decrypter = XMASDecrypter::new();

    xmas_decrypter.find_first_encoding_error_instance(&xmas_encrypted_messages, preamble_length)
}

pub fn encryption_weakness(
    xmas_encrypted_message_strings: Vec<String>,
    preamble_length: usize,
) -> anyhow::Result<u64> {
    let xmas_encrypted_messages: Vec<u64> = xmas_encrypted_message_strings
        .iter()
        .map(|s| s.parse())
        .collect::<Result<Vec<u64>, ParseIntError>>()?;
    let xmas_decrypter = XMASDecrypter::new();

    xmas_decrypter.get_encryption_weakness(&xmas_encrypted_messages, preamble_length)
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use super::*;

    #[test]
    fn finds_first_xmas_encoding_error() {
        let xmas_encrypted_message_strings = vec![
            "35", "20", "15", "25", "47", "40", "62", "55", "65", "95", "102", "117", "150", "182",
            "127", "219", "299", "277", "309", "576",
        ]
        .iter()
        .map(ToString::to_string)
        .collect();

        assert_that(&first_xmas_encoding_error(xmas_encrypted_message_strings, 5).unwrap())
            .is_equal_to(127);
    }

    #[test]
    fn gets_encryption_weakness() {
        let xmas_encrypted_message_strings = vec![
            "35", "20", "15", "25", "47", "40", "62", "55", "65", "95", "102", "117", "150", "182",
            "127", "219", "299", "277", "309", "576",
        ]
        .iter()
        .map(ToString::to_string)
        .collect();

        assert_that(&encryption_weakness(xmas_encrypted_message_strings, 5).unwrap())
            .is_equal_to(62);
    }
}
