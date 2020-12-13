use crate::challenges::challenge_command;

#[test]
fn day_13_part_1() {
    let mut cmd = challenge_command(13, 1);

    cmd.assert().success().stdout("Answer: 2382\n");
}

#[test]
fn day_13_part_2() {
    let mut cmd = challenge_command(13, 2);

    cmd.assert().success().stdout("Answer: 906332393333683\n");
}
