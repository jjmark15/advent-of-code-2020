use crate::challenges::challenge_command;

#[test]
fn day_12_part_1() {
    let mut cmd = challenge_command(12, 1);

    cmd.assert().success().stdout("Answer: 757\n");
}