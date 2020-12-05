use std::path::PathBuf;

use structopt::StructOpt;

use advent_of_code_2020::challenge::Challenge;

#[derive(StructOpt, Debug)]
#[structopt(name = "Advent of Code 2020")]
pub struct Opt {
    /// Choose challenge
    #[structopt(short, long, default_value = "1.1")]
    challenge: Challenge,

    /// Input file
    #[structopt(short, long, parse(from_os_str))]
    input: PathBuf,
}

impl Opt {
    pub fn challenge(&self) -> Challenge {
        self.challenge
    }

    pub fn input(&self) -> PathBuf {
        self.input.clone()
    }
}
