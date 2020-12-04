use advent_of_code_2020::challenge::Challenge;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Advent of Code 2020")]
pub struct Opt {
    /// Choose challenge
    #[structopt(short, long, default_value = "1.1")]
    challenge: Challenge,
}

impl Opt {
    pub fn challenge(&self) -> Challenge {
        self.challenge
    }
}
