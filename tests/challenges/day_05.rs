use crate::challenges::challenge_command;

#[test]
fn day_05_part_1() {
    let mut cmd = challenge_command(5, 1);

    cmd.assert().success().stdout("Answer: 896\n");
}
