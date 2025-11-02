//! Find the number of multiples of 3 or 5 under 1000.

use std::collections::HashSet;

pub fn solve() -> u64 {
    const MAX: u64 = 1000;

    multiples_under(3, MAX)
        .union(&multiples_under(5, MAX))
        .into_iter()
        .sum()
}

/// The multiples of the number `of` that are less than `n`.
fn multiples_under(of: u64, n: u64) -> HashSet<u64> {
    let mut set = HashSet::new();
    let mut temp = of;
    while temp < n {
        set.insert(temp);
        temp += of;
    }
    set
}
