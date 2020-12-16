use crate::challenges::challenge_command;

#[test]
fn day_16_part_1() {
    let mut cmd = challenge_command(16, 1);

    cmd.assert().success().stdout("Answer: 29878\n");
}
