//! Find the sum of all fibonacci numbers under 4M.

pub fn solve() -> u64 {
    const MAX: u64 = 4_000_000;

    let mut back_two = 0;
    let mut back_one = 1;
    let mut sum = 0;

    while back_one < MAX {
        if back_one % 2 == 0 {
            sum += back_one;
        }
        let next = back_one + back_two;
        back_two = back_one;
        back_one = next;
    }
    sum
}
