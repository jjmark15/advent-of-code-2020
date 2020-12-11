use std::collections::hash_set::HashSet;
use std::iter::FromIterator;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Answer(char);

#[derive(Debug)]
struct IndividualMemberPositiveAnswerSet {
    answers: HashSet<Answer>,
}

impl IndividualMemberPositiveAnswerSet {
    fn new(answers: HashSet<Answer>) -> Self {
        IndividualMemberPositiveAnswerSet { answers }
    }

    fn answers(&self) -> &HashSet<Answer> {
        &self.answers
    }
}

struct TravelGroupPositiveAnswerSet {
    answers: HashSet<Answer>,
}

impl TravelGroupPositiveAnswerSet {
    fn from_unifying_individual_member_answer_sets(
        individual_member_answer_sets: Vec<IndividualMemberPositiveAnswerSet>,
    ) -> Self {
        TravelGroupPositiveAnswerSet {
            answers: HashSet::from_iter(
                individual_member_answer_sets
                    .iter()
                    .map(IndividualMemberPositiveAnswerSet::answers)
                    .cloned()
                    .flatten(),
            ),
        }
    }

    fn from_intersecting_individual_member_answer_sets(
        individual_member_answer_sets: Vec<IndividualMemberPositiveAnswerSet>,
    ) -> Self {
        let answer_intersection: HashSet<Answer> = individual_member_answer_sets
            .iter()
            .map(IndividualMemberPositiveAnswerSet::answers)
            .fold(
                individual_member_answer_sets
                    .first()
                    .unwrap()
                    .answers()
                    .clone(),
                |accumulator, answers| accumulator.intersection(answers).cloned().collect(),
            );

        TravelGroupPositiveAnswerSet {
            answers: answer_intersection,
        }
    }

    fn len(&self) -> usize {
        self.answers.len()
    }
}

fn travel_group_answer_sets_from_string<S: AsRef<str>>(
    answer_sets_string: S,
) -> Vec<IndividualMemberPositiveAnswerSet> {
    answer_sets_string
        .as_ref()
        .lines()
        .map(|line| HashSet::from_iter(line.chars().map(Answer)))
        .map(IndividualMemberPositiveAnswerSet::new)
        .collect()
}

pub fn count_total_group_unified_positive_answers(answer_groups: Vec<String>) -> usize {
    answer_groups
        .iter()
        .map(travel_group_answer_sets_from_string)
        .map(TravelGroupPositiveAnswerSet::from_unifying_individual_member_answer_sets)
        .map(|group_set| group_set.len())
        .sum()
}

pub fn count_total_group_intersecting_positive_answers(answer_groups: Vec<String>) -> usize {
    answer_groups
        .iter()
        .map(travel_group_answer_sets_from_string)
        .map(TravelGroupPositiveAnswerSet::from_intersecting_individual_member_answer_sets)
        .map(|group_set| group_set.len())
        .sum()
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use super::*;

    #[test]
    fn counts_total_group_unified_positive_answers() {
        let answer_groups: Vec<String> = vec!["abc", "a\nb\nc", "ab\nac", "a\na\na\na", "b"]
            .iter()
            .map(ToString::to_string)
            .collect();

        assert_that(&count_total_group_unified_positive_answers(answer_groups)).is_equal_to(11);
    }

    #[test]
    fn counts_total_group_intersecting_positive_answers() {
        let answer_groups: Vec<String> = vec!["abc", "a\nb\nc", "ab\nac", "a\na\na\na", "b"]
            .iter()
            .map(ToString::to_string)
            .collect();

        assert_that(&count_total_group_intersecting_positive_answers(
            answer_groups,
        ))
        .is_equal_to(6);
    }
}
