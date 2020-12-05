use crate::challenges::challenge_command;

#[test]
fn day_01_part_1() {
    let mut cmd = challenge_command(1, 1);

    cmd.assert().success().stdout("Answer: 793524\n");
}

#[test]
fn day_01_part_2() {
    let mut cmd = challenge_command(1, 2);

    cmd.assert().success().stdout("Answer: 61515678\n");
}
