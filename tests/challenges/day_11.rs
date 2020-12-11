use crate::challenges::challenge_command;

#[test]
fn day_11_part_1() {
    let mut cmd = challenge_command(11, 1);

    cmd.assert().success().stdout("Answer: 2386\n");
}
