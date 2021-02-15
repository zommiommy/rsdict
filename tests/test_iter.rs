// other library used for correctness checks
extern crate fid;
use fid::{FID, BitVector};

fn xorshift(mut x: u64) -> u64 {
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    x
}

const SIZE: usize = 1_000_000;

#[test]
/// Check that elias fano runs considering a lot of possible combinations.
fn test_iter() {
    let mut r = rsdict::RsDict::new();
    let mut seed = 0xdeadbeef;

    let mut vector = Vec::with_capacity(SIZE);

    for _ in 0..SIZE {
        seed = xorshift(seed);
        let val = (seed & 1) == 1;
        vector.push(val);
        r.push(val)
    }

    let indices: Vec<u64> = vector.iter().enumerate().filter(|(i, x)| **x).map(|(i, x)| i as u64).collect();

    assert_eq!(indices, r.iter().collect::<Vec<u64>>());;
}