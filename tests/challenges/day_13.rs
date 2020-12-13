use crate::challenges::challenge_command;

#[test]
fn day_13_part_1() {
    let mut cmd = challenge_command(13, 1);

    cmd.assert().success().stdout("Answer: 2382\n");
}
