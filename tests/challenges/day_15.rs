use crate::challenges::challenge_command;

#[test]
fn day_15_part_1() {
    let mut cmd = challenge_command(15, 1);

    cmd.assert().success().stdout("Answer: 610\n");
}

#[test]
fn day_15_part_2() {
    let mut cmd = challenge_command(15, 2);

    cmd.assert().success().stdout("Answer: 1407\n");
}
