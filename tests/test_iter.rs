// other library used for correctness checks
extern crate fid;

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

    assert_eq!(indices, r.iter().collect::<Vec<u64>>());
}

#[test]
/// Check that elias fano runs considering a lot of possible combinations.
fn test_iter_in_range() {
    let mut r = rsdict::RsDict::new();
    let mut seed = 0xc0febeef;
    let size = 100usize;

    for _ in 0..100 {
        let mut vector = Vec::with_capacity(size);
        let mut values = Vec::with_capacity(size);

        for i in 0..size {
            seed = xorshift(seed);
            let val = (seed & 1) == 1;
            vector.push(val);
            r.push(val);
            if val {
                values.push(i);
            }
        }

        seed = xorshift(seed);
        let start = seed % (size + 10) as u64;
        let mut end = start;
        while end <= start {
            seed = xorshift(seed);
            end = seed % (size + 10) as u64;
        }

        let indices: Vec<usize> = values.iter()
            .filter(|x| (start..end).contains(&(**x as u64)))
            .cloned().collect();

        let result: Vec<u64> = r.iter_in_range(start as u64..end as u64)
            .unwrap().collect();

        println!("$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$");
        println!("Start {:10} End {:10}", start, end);
        println!("Values: {:?}", values);
        println!("Truth:  {:?}", indices);
        println!("Result: {:?}", result);

        assert_eq!(indices.len(), result.len());
        for (a, b) in indices.iter().zip(result.iter()) {
            assert_eq!(*a, *b as usize);
        }
    }
}