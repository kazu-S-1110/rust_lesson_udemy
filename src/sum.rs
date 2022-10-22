pub fn sum(targetNum: u32) -> u32 {
    if targetNum == 0 {
        0;
    }

    (targetNum * (targetNum + 1)) / 2
}
