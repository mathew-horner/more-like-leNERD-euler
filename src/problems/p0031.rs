//! Count number of ways to combine the given coins to equal 2£.

use std::collections::HashSet;

const COINS: [usize; 8] = [1, 2, 5, 10, 20, 50, 100, 200];

pub fn solve() -> usize {
    let mut combos = 0;
    solve_inner(0, &mut HashSet::new(), &mut combos);
    combos
}

fn solve_inner(combo: u64, seen: &mut HashSet<u64>, ways: &mut usize) {
    if seen.contains(&combo) {
        return;
    }
    seen.insert(combo);
    let pence = pence(combo);
    if pence == 200 {
        *ways += 1;
        return;
    }
    if pence > 200 {
        return;
    }
    for idx in 0..COINS.len() {
        solve_inner(increment_coin(combo, idx), seen, ways);
    }
}

fn increment_coin(mut combo: u64, coin_idx: usize) -> u64 {
    let shift = 8 * coin_idx;
    let mut count = (combo >> shift) & 0xFF;
    count += 1;
    combo &= !(0xFF << shift);
    combo |= count << shift;
    combo
}

fn pence(mut combo: u64) -> usize {
    let mut pence = 0;
    for coin_idx in 0..COINS.len() {
        pence += (combo & 0xFF) as usize * COINS[coin_idx];
        combo >>= 8;
    }
    pence
}
