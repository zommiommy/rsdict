#![no_main]
use libfuzzer_sys::fuzz_target;
use fid::{FID, BitVector};

fuzz_target!(|data: Vec<u16>| {
    let mut data = data.clone();
    // create a sorted vector with no duplicates
    data.sort();
    data.dedup();

    let mut r = rsdict::RsDict::new();
    let mut bv = BitVector::new();

    let mut last_value =  0;
    for v in &data {
        for _ in last_value..*v {
            r.push(false);
            bv.push(false);
        }
        if *v != last_value {
            r.push(true);
            bv.push(true);
        }
        last_value = *v;
    }

    assert_eq!(bv.len() as usize, r.len(), "the length of the vector do not match!");
    
    for i in 0..r.count_ones() {
        assert_eq!(r.select1(i as u64).unwrap(), bv.select1(i as u64));
    }

    for i in 0..r.count_zeros() {
        assert_eq!(r.select0(i as u64).unwrap(), bv.select0(i as u64));
    }

    for i in 0..r.len() {
        assert_eq!(r.rank(i as u64, true), bv.rank1(i as u64));
    }


});
