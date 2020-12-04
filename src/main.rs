use std::io;
use std::io::BufRead;
use std::num::ParseIntError;

use structopt::StructOpt;

use advent_of_code_2020::answer::Answer;
use advent_of_code_2020::challenge::{Challenge, ChallengePart};
use advent_of_code_2020::MovementDirection::{Down, Right};
use advent_of_code_2020::{
    count_encountered_trees_for_movement_sequence, count_policies_satisfied_by_passwords,
    product_of_2020_sum_pair, product_of_2020_sum_triplet, to_policy_and_password,
    OccurrenceRestrictedPasswordPolicy, Password, PositionallyRestrictedPasswordPolicy,
};

use crate::cli::Opt;

mod cli;

fn main() {
    let opt = Opt::from_args();
    let challenge = opt.challenge();

    execute_challenge(challenge);
}

fn execute_challenge(challenge: Challenge) {
    match challenge.day() {
        1 => run_day_1(challenge.part()).unwrap(),
        2 => run_day_2(challenge.part()).unwrap(),
        3 => run_day_3(challenge.part()).unwrap(),
        _ => unimplemented!(),
    }
}

fn run_day_1(part: ChallengePart) -> anyhow::Result<()> {
    let input = read_input("Enter numbers:");
    let numbers: Vec<u64> = input
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

fn run_day_2(part: ChallengePart) -> anyhow::Result<()> {
    let input = read_input("Enter password policies and passwords:");

    let result: usize = match part {
        ChallengePart::One => count_policies_satisfied_by_passwords(
            input
                .iter()
                .map(to_policy_and_password)
                .collect::<anyhow::Result<Vec<(OccurrenceRestrictedPasswordPolicy, Password)>>>()?,
        ),
        ChallengePart::Two => count_policies_satisfied_by_passwords(
            input
                .iter()
                .map(to_policy_and_password)
                .collect::<anyhow::Result<Vec<(PositionallyRestrictedPasswordPolicy, Password)>>>(
                )?,
        ),
    };

    println!("{}", Answer::new(result));
    Ok(())
}

fn run_day_3(part: ChallengePart) -> anyhow::Result<()> {
    let input = read_input("Enter password policies and passwords:");

    let result: u16 = match part {
        ChallengePart::One => {
            count_encountered_trees_for_movement_sequence(input, vec![Right, Right, Right, Down])
        }
        ChallengePart::Two => unimplemented!(),
    };

    println!("{}", Answer::new(result));
    Ok(())
}

fn read_input(prompt: &str) -> Vec<String> {
    println!("{}", prompt);
    let stdin = io::stdin();

    stdin
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .take_while(|l| !l.is_empty())
        .collect()
}
