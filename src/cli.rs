use structopt::StructOpt;


#[derive(StructOpt, Debug)]
#[structopt(name = "Advent of Code 2020")]
pub struct Opt {
    /// Choose day
    #[structopt(short, long, default_value = "1")]
    day: u8
}

impl Opt {
    pub fn day(&self) -> u8 {
        self.day
    }
}