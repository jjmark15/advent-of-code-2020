use crate::challenges::challenge_command;

#[test]
fn day_09_part_1() {
    let mut cmd = challenge_command(9, 1);

    cmd.assert().success().stdout("Answer: 29221323\n");
}
