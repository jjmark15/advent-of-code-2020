use crate::challenges::challenge_command;

#[test]
fn day_02_part_1() {
    let mut cmd = challenge_command(2, 1);

    cmd.assert().success().stdout("Answer: 393\n");
}

#[test]
fn day_02_part_2() {
    let mut cmd = challenge_command(2, 2);

    cmd.assert().success().stdout("Answer: 690\n");
}
