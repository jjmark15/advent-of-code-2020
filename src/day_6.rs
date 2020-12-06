use std::collections::hash_set::HashSet;
use std::iter::FromIterator;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Answer(char);

struct DistinctPositiveAnswerSet {
    answers: HashSet<Answer>,
}

impl DistinctPositiveAnswerSet {
    fn new(answers: HashSet<Answer>) -> Self {
        DistinctPositiveAnswerSet { answers }
    }

    fn answers(&self) -> &HashSet<Answer> {
        &self.answers
    }
}

struct TravelGroupDistinctPositiveAnswerSet {
    answers: HashSet<Answer>,
}

impl TravelGroupDistinctPositiveAnswerSet {
    fn from_indistinct_answer_sets(answer_sets: Vec<DistinctPositiveAnswerSet>) -> Self {
        TravelGroupDistinctPositiveAnswerSet {
            answers: HashSet::from_iter(
                answer_sets
                    .iter()
                    .map(DistinctPositiveAnswerSet::answers)
                    .map(Clone::clone)
                    .flatten(),
            ),
        }
    }

    fn len(&self) -> usize {
        self.answers.len()
    }
}

fn travel_group_answer_sets_from_string<S: AsRef<str>>(
    answer_sets_string: S,
) -> Vec<DistinctPositiveAnswerSet> {
    answer_sets_string
        .as_ref()
        .lines()
        .map(|line| HashSet::from_iter(line.chars().map(Answer)))
        .map(DistinctPositiveAnswerSet::new)
        .collect()
}

pub fn count_total_group_distinct_positive_answers(group_answers: Vec<String>) -> usize {
    group_answers
        .iter()
        .map(travel_group_answer_sets_from_string)
        .map(TravelGroupDistinctPositiveAnswerSet::from_indistinct_answer_sets)
        .map(|group_set| group_set.len())
        .sum()
}
