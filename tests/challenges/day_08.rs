use crate::challenges::challenge_command;

#[test]
fn day_08_part_1() {
    let mut cmd = challenge_command(8, 1);

    cmd.assert().success().stdout("Answer: 1766\n");
}

#[test]
fn day_08_part_2() {
    let mut cmd = challenge_command(8, 2);

    cmd.assert().success().stdout("Answer: 1639\n");
}
