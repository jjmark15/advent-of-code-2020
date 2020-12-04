fn sum_to_2020(a: u64, b: u64) -> bool {
    a + b == 2020
}

fn find_2020_sum_pair(numbers: Vec<u64>) -> Option<(u64, u64)> {
    for a in &numbers {
        for b in &numbers {
            if sum_to_2020(*a, *b) {
                return Some((*a, *b));
            }
        }
    }
    None
}

pub fn product_of_2020_sum_pair(numbers: Vec<u64>) -> Option<u64> {
    match find_2020_sum_pair(numbers) {
        Some((a, b)) => Some(a * b),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn finds_pair_that_sums_to_2020() {
        let numbers = vec![1721, 979, 366, 299, 675, 1456];

        assert_that(&product_of_2020_sum_pair(numbers).unwrap()).is_equal_to(299 * 1721);
    }
}
