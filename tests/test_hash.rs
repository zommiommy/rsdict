// other library used for correctness checks
extern crate fid;
use fid::{FID, BitVector};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn xorshift(mut x: u64) -> u64 {
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    x
}

const SIZE: usize = 1_000_000;

#[test]
/// Check that the hash is consistent
fn test_hash() {
    let mut r1 = rsdict::RsDict::new();
    let mut r2 = rsdict::RsDict::new();

    let mut seed = 0xdeadbeef;

    for _ in 0..SIZE {
        seed = xorshift(seed);
        let val = (seed & 1) == 1;
        r1.push(val);
        r2.push(val);
    }

    let mut hasher = DefaultHasher::new();
    r1.hash(&mut hasher);
    let h1 = hasher.finish();

    let mut hasher = DefaultHasher::new();
    r2.hash(&mut hasher);
    let h2 = hasher.finish();

    assert_eq!(h1, h2);

}