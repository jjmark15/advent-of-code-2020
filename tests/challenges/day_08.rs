use crate::challenges::challenge_command;

#[test]
fn day_07_part_1() {
    let mut cmd = challenge_command(8, 1);

    cmd.assert().success().stdout("Answer: 1766\n");
}
