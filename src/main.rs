use std::fs;
use std::num::ParseIntError;
use std::path::PathBuf;

use structopt::StructOpt;

use advent_of_code_2020::answer::Answer;
use advent_of_code_2020::challenge::{Challenge, ChallengePart};
use advent_of_code_2020::day_1::{product_of_2020_sum_pair, product_of_2020_sum_triplet};
use advent_of_code_2020::day_2::{
    count_policies_satisfied_by_passwords, to_policy_and_password,
    OccurrenceRestrictedPasswordPolicy, Password, PositionallyRestrictedPasswordPolicy,
};
use advent_of_code_2020::day_3::MovementDirection::{Down, Right};
use advent_of_code_2020::day_3::{
    count_encountered_trees_for_movement_sequence,
    product_of_tree_encounters_for_movement_sequences,
};
use advent_of_code_2020::day_4::{
    count_valid_relaxed_validation_passports_in_text,
    count_valid_strict_validation_passports_in_text,
};

use crate::cli::Opt;

mod cli;

fn main() {
    let opt = Opt::from_args();
    let challenge = opt.challenge();
    let input_text_lines = read_input_file(opt.input()).unwrap();

    execute_challenge(challenge, input_text_lines);
}

fn execute_challenge(challenge: Challenge, input_text_lines: Vec<String>) {
    match challenge.day() {
        1 => run_day_1(challenge.part(), input_text_lines).unwrap(),
        2 => run_day_2(challenge.part(), input_text_lines).unwrap(),
        3 => run_day_3(challenge.part(), input_text_lines).unwrap(),
        4 => run_day_4(challenge.part(), input_text_lines).unwrap(),
        _ => unimplemented!(),
    }
}

fn run_day_1(part: ChallengePart, input_text_lines: Vec<String>) -> anyhow::Result<()> {
    let numbers: Vec<u64> = input_text_lines
        .iter()
        .map(|s| s.as_str().parse())
        .collect::<Result<Vec<u64>, ParseIntError>>()?;

    let result = match part {
        ChallengePart::One => product_of_2020_sum_pair(&*numbers).unwrap(),
        ChallengePart::Two => product_of_2020_sum_triplet(&*numbers).unwrap(),
    };

    println!("{}", Answer::new(result));
    Ok(())
}

fn run_day_2(part: ChallengePart, input_text_lines: Vec<String>) -> anyhow::Result<()> {
    let result: usize = match part {
        ChallengePart::One => count_policies_satisfied_by_passwords(
            input_text_lines
                .iter()
                .map(to_policy_and_password)
                .collect::<anyhow::Result<Vec<(OccurrenceRestrictedPasswordPolicy, Password)>>>()?,
        ),
        ChallengePart::Two => count_policies_satisfied_by_passwords(
            input_text_lines
                .iter()
                .map(to_policy_and_password)
                .collect::<anyhow::Result<Vec<(PositionallyRestrictedPasswordPolicy, Password)>>>(
                )?,
        ),
    };

    println!("{}", Answer::new(result));
    Ok(())
}

fn run_day_3(part: ChallengePart, input_text_lines: Vec<String>) -> anyhow::Result<()> {
    let result: u64 = match part {
        ChallengePart::One => count_encountered_trees_for_movement_sequence(
            input_text_lines,
            vec![Right, Right, Right, Down],
        ),
        ChallengePart::Two => product_of_tree_encounters_for_movement_sequences(
            input_text_lines,
            vec![
                vec![Right, Down],
                vec![Right, Right, Right, Down],
                vec![Right, Right, Right, Right, Right, Down],
                vec![Right, Right, Right, Right, Right, Right, Right, Down],
                vec![Right, Down, Down],
            ],
        ),
    };

    println!("{}", Answer::new(result));
    Ok(())
}

fn run_day_4(part: ChallengePart, input_text_lines: Vec<String>) -> anyhow::Result<()> {
    let passport_strings: Vec<String> = input_text_lines
        .join("\n")
        .split("\n\n")
        .map(ToString::to_string)
        .collect();

    let result: usize = match part {
        ChallengePart::One => count_valid_relaxed_validation_passports_in_text(passport_strings),
        ChallengePart::Two => count_valid_strict_validation_passports_in_text(passport_strings),
    };

    println!("{}", Answer::new(result));
    Ok(())
}

fn read_input_file(p: PathBuf) -> anyhow::Result<Vec<String>> {
    let file_string = fs::read_to_string(p)?.trim().to_string();
    Ok(file_string.lines().map(ToString::to_string).collect())
}
