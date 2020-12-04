fn find_2020_sum_pair(numbers: &[u64]) -> Option<(u64, u64)> {
    for a in numbers {
        for b in numbers {
            if a + b == 2020 {
                return Some((*a, *b));
            }
        }
    }
    None
}

fn find_2020_sum_triplet(numbers: &[u64]) -> Option<(u64, u64, u64)> {
    for a in numbers {
        for b in numbers {
            for c in numbers {
                if a + b + c == 2020 {
                    return Some((*a, *b, *c));
                }
            }
        }
    }
    None
}

pub fn product_of_2020_sum_pair(numbers: &[u64]) -> Option<u64> {
    match find_2020_sum_pair(numbers) {
        Some((a, b)) => Some(a * b),
        None => None,
    }
}

pub fn product_of_2020_sum_triplet(numbers: &[u64]) -> Option<u64> {
    match find_2020_sum_triplet(numbers) {
        Some((a, b, c)) => Some(a * b * c),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use super::*;

    #[test]
    fn finds_pair_that_sums_to_2020() {
        let numbers = vec![1721, 979, 366, 299, 675, 1456];

        assert_that(&product_of_2020_sum_pair(&*numbers).unwrap()).is_equal_to(299 * 1721);
    }

    #[test]
    fn finds_triplet_that_sums_to_2020() {
        let numbers = vec![1721, 979, 366, 299, 675, 1456];

        assert_that(&product_of_2020_sum_triplet(&*numbers).unwrap()).is_equal_to(979 * 366 * 675);
    }
}
