use crate::challenges::challenge_command;

#[test]
fn day_06_part_1() {
    let mut cmd = challenge_command(6, 1);

    cmd.assert().success().stdout("Answer: 6703\n");
}
