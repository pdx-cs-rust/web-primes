pub use num_bigint::BigUint;

use glass_pumpkin::prime::check;
use num_bigint::RandBigInt;

/// Produce a randomly-chosen prime in the range `2**n-1..=2**n`.
// Strategy: try random integers in-range forced to odd
// until one of them passes the `check()` test.
pub fn prime(nbits: u32) -> BigUint {
    // https://stackoverflow.com/a/56404365
    let mut rng = rand::thread_rng();
    let max: BigUint = BigUint::from(1u32) << nbits;
    let min = max.clone() >> 1u32;
    loop {
        let p = rng.gen_biguint_range(&min, &max) | BigUint::from(1u32);
        if check(&p) {
            return p;
        }
    }
}
