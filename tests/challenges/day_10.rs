use crate::challenges::challenge_command;

#[test]
fn day_10_part_1() {
    let mut cmd = challenge_command(10, 1);

    cmd.assert().success().stdout("Answer: 3034\n");
}
