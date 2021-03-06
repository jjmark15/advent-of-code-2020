use std::path::PathBuf;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_15;
mod day_16;

fn challenge_command(day: u8, part: u8) -> assert_cmd::Command {
    let mut cmd = assert_cmd::Command::cargo_bin("advent-of-code-2020").unwrap();

    cmd.args(&["-c", format!("{}.{}", day, part).as_str()]);

    let sample_data_path: PathBuf = ["sample_data", sample_data_file_name(day).as_str()]
        .iter()
        .collect();
    cmd.args(&["-i", sample_data_path.as_os_str().to_str().unwrap()]);

    cmd
}

fn sample_data_file_name(day: u8) -> String {
    format!("day_{:02}.txt", day)
}

fn assert_challenge_result(day: u8, part: u8, result: &str) {
    let mut cmd = challenge_command(day, part);

    cmd.assert()
        .success()
        .stdout(format!("Answer: {}\n", result));
}
