use crate::cli::Opt;
use advent_of_code_2020::answer::Answer;
use advent_of_code_2020::product_of_2020_sum_pair;
use std::io;
use std::io::BufRead;
use std::num::ParseIntError;
use structopt::StructOpt;

mod cli;

fn main() {
    let opt = Opt::from_args();

    match opt.day() {
        1 => run_day_1().unwrap(),
        _ => unimplemented!(),
    }
}

fn run_day_1() -> anyhow::Result<()> {
    let input = read_input("Enter numbers:");
    let numbers: Vec<u64> = input
        .iter()
        .map(|s| s.as_str().parse())
        .collect::<Result<Vec<u64>, ParseIntError>>()?;

    let result: Answer<u64> = Answer::new(product_of_2020_sum_pair(numbers).unwrap());

    println!("{}", result);
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
