#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: Vec<u16>| {
    let mut data = data.clone();
    // create a sorted vector with no duplicates
    data.sort();
    data.dedup();

    let mut r = rsdict::RsDict::new();

    let mut last_value = 0;
    for v in &data {
        for _ in last_value..*v {
            r.push(false);
        }
        r.push(true);
        last_value = *v;
    }

    //println!("data: {:?}", &data);

    let truth = (0..data.len()).map(|i| r.select1(i as u64).unwrap()).collect::<Vec<u64>>();

    let iter = r.iter();
    //println!("iter: {:?}", iter);
    let result: Vec<u64> = iter.collect();

    //println!("truth: {:?}", &truth);
    //println!("ours : {:?}", &result);

    assert_eq!(truth.len(), result.len(), "the length of the vector do not match!");
    for (a, b) in truth.iter().zip(result.iter()) {
        assert_eq!(*a as usize, *b as usize, "the values of the vectors do not match!");
    }

});
