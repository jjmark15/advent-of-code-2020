use crate::challenges::challenge_command;

#[test]
fn day_07_part_1() {
    let mut cmd = challenge_command(7, 1);

    cmd.assert().success().stdout("Answer: 155\n");
}
