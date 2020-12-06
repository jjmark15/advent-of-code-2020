use crate::challenges::challenge_command;

#[test]
fn day_06_part_1() {
    let mut cmd = challenge_command(6, 1);

    cmd.assert().success().stdout("Answer: 6703\n");
}

#[test]
fn day_06_part_2() {
    let mut cmd = challenge_command(6, 2);

    cmd.assert().success().stdout("Answer: 3430\n");
}
