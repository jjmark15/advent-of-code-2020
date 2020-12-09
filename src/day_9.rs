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

    fn find_first_weakness_instance(
        &self,
        xmas_encrypted_messages: Vec<u64>,
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
}

pub fn find_first_xmas_encoding_error(
    xmas_encrypted_message_strings: Vec<String>,
    preamble_length: usize,
) -> anyhow::Result<u64> {
    let xmas_encrypted_messages: Vec<u64> = xmas_encrypted_message_strings
        .iter()
        .map(|s| s.parse())
        .collect::<Result<Vec<u64>, ParseIntError>>()?;
    let xmas_decrypter = XMASDecrypter::new();

    xmas_decrypter.find_first_weakness_instance(xmas_encrypted_messages, preamble_length)
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

        assert_that(&find_first_xmas_encoding_error(xmas_encrypted_message_strings, 5).unwrap())
            .is_equal_to(127);
    }
}
