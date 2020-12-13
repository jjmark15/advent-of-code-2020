use std::fs;
use std::num::ParseIntError;
use std::path::PathBuf;

use structopt::StructOpt;

use advent_of_code_2020::answer::Answer;
use advent_of_code_2020::challenge::{Challenge, ChallengePart};
use advent_of_code_2020::day_01::{product_of_2020_sum_pair, product_of_2020_sum_triplet};
use advent_of_code_2020::day_02::{
    count_policies_satisfied_by_passwords, to_policy_and_password,
    OccurrenceRestrictedPasswordPolicy, Password, PositionallyRestrictedPasswordPolicy,
};
use advent_of_code_2020::day_03::MovementDirection::{Down, Right};
use advent_of_code_2020::day_03::{
    count_encountered_trees_for_movement_sequence,
    product_of_tree_encounters_for_movement_sequences,
};
use advent_of_code_2020::day_04::{
    count_valid_relaxed_validation_passports_in_text,
    count_valid_strict_validation_passports_in_text,
};
use advent_of_code_2020::day_05::{find_my_empty_seat_id, highest_seat_id_on_plane};
use advent_of_code_2020::day_06::{
    total_group_intersecting_positive_answers, total_group_unified_positive_answers,
};
use advent_of_code_2020::day_07::count_bags_that_eventually_contain;
use advent_of_code_2020::day_08::{
    accumulator_value_after_termination_of_fixed_instructions,
    accumulator_value_before_repeated_instruction,
};
use advent_of_code_2020::day_09::{encryption_weakness, first_xmas_encoding_error};
use advent_of_code_2020::day_10::product_of_1_and_3_joltage_differences_using_every_adapter_and_built_in;
use advent_of_code_2020::day_11::count_occupied_seats_after_occupancy_stabilisation;
use advent_of_code_2020::day_12::{
    manhattan_distance_to_directed_location,
    manhattan_distance_to_directed_location_with_waypoint_navigation,
};
use advent_of_code_2020::day_13::{
    earliest_timestamp_such_that_all_listed_buses_depart_at_offsets_matching_their_positions,
    product_of_id_of_earliest_bus_and_wait_time,
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
        5 => run_day_5(challenge.part(), input_text_lines).unwrap(),
        6 => run_day_6(challenge.part(), input_text_lines).unwrap(),
        7 => run_day_7(challenge.part(), input_text_lines).unwrap(),
        8 => run_day_8(challenge.part(), input_text_lines).unwrap(),
        9 => run_day_9(challenge.part(), input_text_lines).unwrap(),
        10 => run_day_10(challenge.part(), input_text_lines).unwrap(),
        11 => run_day_11(challenge.part(), input_text_lines).unwrap(),
        12 => run_day_12(challenge.part(), input_text_lines).unwrap(),
        13 => run_day_13(challenge.part(), input_text_lines).unwrap(),
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

fn run_day_5(part: ChallengePart, input_text_lines: Vec<String>) -> anyhow::Result<()> {
    let result = match part {
        ChallengePart::One => highest_seat_id_on_plane(input_text_lines)?,
        ChallengePart::Two => find_my_empty_seat_id(input_text_lines)?,
    };

    println!("{}", Answer::new(result));
    Ok(())
}

fn run_day_6(part: ChallengePart, input_text_lines: Vec<String>) -> anyhow::Result<()> {
    let passport_strings: Vec<String> = input_text_lines
        .join("\n")
        .split("\n\n")
        .map(ToString::to_string)
        .collect();

    let result: usize = match part {
        ChallengePart::One => total_group_unified_positive_answers(passport_strings),
        ChallengePart::Two => total_group_intersecting_positive_answers(passport_strings),
    };

    println!("{}", Answer::new(result));
    Ok(())
}

fn run_day_7(part: ChallengePart, input_text_lines: Vec<String>) -> anyhow::Result<()> {
    let result: usize = match part {
        ChallengePart::One => count_bags_that_eventually_contain(input_text_lines, "shiny gold")?,
        ChallengePart::Two => unimplemented!(),
    };

    println!("{}", Answer::new(result));
    Ok(())
}

fn run_day_8(part: ChallengePart, input_text_lines: Vec<String>) -> anyhow::Result<()> {
    let result: i64 = match part {
        ChallengePart::One => accumulator_value_before_repeated_instruction(input_text_lines)?,
        ChallengePart::Two => {
            accumulator_value_after_termination_of_fixed_instructions(input_text_lines)?
        }
    };

    println!("{}", Answer::new(result));
    Ok(())
}

fn run_day_9(part: ChallengePart, input_text_lines: Vec<String>) -> anyhow::Result<()> {
    let result: u64 = match part {
        ChallengePart::One => first_xmas_encoding_error(input_text_lines, 25)?,
        ChallengePart::Two => encryption_weakness(input_text_lines, 25)?,
    };

    println!("{}", Answer::new(result));
    Ok(())
}

fn run_day_10(part: ChallengePart, input_text_lines: Vec<String>) -> anyhow::Result<()> {
    let result: u64 = match part {
        ChallengePart::One => {
            product_of_1_and_3_joltage_differences_using_every_adapter_and_built_in(
                input_text_lines,
            )?
        }
        ChallengePart::Two => unimplemented!(),
    };

    println!("{}", Answer::new(result));
    Ok(())
}

fn run_day_11(part: ChallengePart, input_text_lines: Vec<String>) -> anyhow::Result<()> {
    let result: usize = match part {
        ChallengePart::One => count_occupied_seats_after_occupancy_stabilisation(input_text_lines)?,
        ChallengePart::Two => unimplemented!(),
    };

    println!("{}", Answer::new(result));
    Ok(())
}

fn run_day_12(part: ChallengePart, input_text_lines: Vec<String>) -> anyhow::Result<()> {
    let result: u64 = match part {
        ChallengePart::One => manhattan_distance_to_directed_location(input_text_lines)?,
        ChallengePart::Two => {
            manhattan_distance_to_directed_location_with_waypoint_navigation(input_text_lines)?
        }
    };

    println!("{}", Answer::new(result));
    Ok(())
}

fn run_day_13(part: ChallengePart, input_text_lines: Vec<String>) -> anyhow::Result<()> {
    let result: u64 = match part {
        ChallengePart::One => product_of_id_of_earliest_bus_and_wait_time(input_text_lines)?,
        ChallengePart::Two => earliest_timestamp_such_that_all_listed_buses_depart_at_offsets_matching_their_positions(input_text_lines)?,
    };

    println!("{}", Answer::new(result));
    Ok(())
}

fn read_input_file(p: PathBuf) -> anyhow::Result<Vec<String>> {
    let file_string = fs::read_to_string(p)?.trim().to_string();
    Ok(file_string.lines().map(ToString::to_string).collect())
}
