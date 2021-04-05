#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::{Arbitrary, Unstructured};


#[derive(Arbitrary, Debug)]
struct InputData {
    start: usize,
    end: usize,
    indices: Vec<u16>,
}


fuzz_target!(|data: &[u8]| {
    let data = InputData::arbitrary(&mut Unstructured::new(data));
    if data.is_err() {
        return;
    }

    let InputData {
        start,
        end,
        mut indices,
    } = data.unwrap();
    // create a sorted vector with no duplicates
    indices.sort();
    indices.dedup();

    //println!("start: {:10} end: {:10} indices: {:?}", start, end, indices);

    let mut r = rsdict::RsDict::new();

    let mut last_value =  0;
    for v in &indices {
        for _ in last_value..*v {
            r.push(false);
        }
        r.push(true);
        last_value = *v;
    }

    let indices: Vec<u64> = r.iter()
        .filter(|x| (start..end).contains(&(*x as usize))).collect();

    let result = r.iter_in_range(start as u64..end as u64);

    //println!("dbug: {:?}", result);

    if result.is_none() {
        assert_eq!(0, indices.len(), "the iter returned non when there are data to be returned");
        return;
    }

    let result: Vec<u64> = result.unwrap().collect();


    //println!("truth: {:?}", &indices);
    //println!("ours : {:?}", &result);

    assert_eq!(indices.len(), result.len(), "the length of the vector do not match!");
    for (a, b) in indices.iter().zip(result.iter()) {
        assert_eq!(*a as usize, *b as usize, "the values of the vectors do not match!");
    }

});
