//! Find the product of the coefficients `a` and `b` that produce the longest consecutive sequence of primes.

use std::collections::HashSet;

// TODO: This is broken somehow...
pub fn solve() -> i64 {
    /// The maximum value of a or b.
    const COEF_MAX: i64 = 1000;

    let mut max = 0;
    let mut max_pair: Option<(i64, i64)> = None;

    for a in (-COEF_MAX + 1)..COEF_MAX {
        for b in (-COEF_MAX)..(COEF_MAX + 1) {
            let result = max_consecutive_primes_for_quadratic(a as i64, b as i64);
            if result > max {
                max = result;
                max_pair = Some((a, b))
            }
        }
    }

    let max_pair = max_pair.expect("Max pair not found, something went wrong...");
    max_pair.0 * max_pair.1
}

/// Count the number of consecutive primes produced by the quadratic expression
/// `n^2 + a*n + b` for `n = 0..`.
fn max_consecutive_primes_for_quadratic(a: i64, b: i64) -> u64 {
    let mut n: u64 = 0;
    loop {
        if !is_prime((n as i64).pow(2) + (a * (n as i64)) + b) {
            break;
        }
        n += 1;
    }
    n
}

/// Whether `n` is a prime number.
fn is_prime(n: i64) -> bool {
    if n < 0 {
        return false;
    }
    factors(n as u64).len() <= 2
}

/// Calculate the factors of `n`.
fn factors(n: u64) -> HashSet<u64> {
    let mut factors = HashSet::new();
    factors.insert(1);
    factors.insert(n);
    for i in 2..((n as f64).sqrt().ceil() as u64) + 1 {
        if n % i == 0 {
            factors.insert(i);
            factors.insert(n / i);
        }
    }
    factors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_prime_works() {
        const N: usize = 20;
        const EXPECTED: [bool; N] = [
            true, true, true, false, true, false, true, false, false, false, true, false, true,
            false, false, false, true, false, true, false,
        ];

        for i in 0..N {
            let n = (i + 1) as i64;
            assert_eq!(is_prime(n), EXPECTED[i], "Failed at n = {}", n);
        }
    }

    #[test]
    fn max_consecutive_primes_for_quadratic_works() {
        const PAIRS: [(i64, i64); 2] = [(1, 41), (-79, 1601)];
        const EXPECTED: [u64; 2] = [40, 80];

        for (i, pair) in PAIRS.iter().enumerate() {
            assert_eq!(
                max_consecutive_primes_for_quadratic(pair.0, pair.1),
                EXPECTED[i],
                "Failed at a = {}, b = {}",
                pair.0,
                pair.1
            );
        }
    }
}
