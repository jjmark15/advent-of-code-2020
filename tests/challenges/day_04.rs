use crate::challenges::challenge_command;

#[test]
fn day_04_part_1() {
    let mut cmd = challenge_command(4, 1);

    cmd.assert().success().stdout("Answer: 208\n");
}

#[test]
fn day_04_part_2() {
    let mut cmd = challenge_command(4, 2);

    cmd.assert().success().stdout("Answer: 167\n");
}
