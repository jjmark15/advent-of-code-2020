use crate::challenges::challenge_command;

#[test]
fn day_03_part_1() {
    let mut cmd = challenge_command(3, 1);

    cmd.assert().success().stdout("Answer: 299\n");
}

#[test]
fn day_03_part_2() {
    let mut cmd = challenge_command(3, 2);

    cmd.assert().success().stdout("Answer: 3621285278\n");
}
