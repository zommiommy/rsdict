use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rsdict::RsDict;
use succinct::bit_vec::{BitVecPush, BitVector};
use succinct::rank::{JacobsonRank, Rank9, RankSupport};
use succinct::select::{BinSearchSelect, Select0Support, Select1Support};

const NUM_BITS: usize = 1_000_000;
const SEED: u64 = 88004802264174740;

fn random_bits(len: usize) -> BitVector<u64> {
    let mut rng = StdRng::seed_from_u64(SEED);
    let mut bv = BitVector::with_capacity(len as u64);
    for _ in 0..len {
        bv.push_bit(rng.gen());
    }
    bv
}

fn random_indices(count: usize, range: usize) -> Vec<usize> {
    let mut rng = StdRng::seed_from_u64(SEED);
    (0..count).map(|_| rng.gen_range(0, range)).collect()
}


fn bench_one_rank<T, F, G>(c: &mut Criterion, name: &str, create: F, rank: G)
    where F: FnOnce(BitVector<u64>) -> T,
          G: Fn(&T, u64) -> u64
{
    let r = create(random_bits(NUM_BITS));
    let indices = random_indices(1000, NUM_BITS);
    c.bench_function(name, |b| {
        b.iter(|| {
            for &ix in &indices {
                rank(&r, black_box(ix as u64));
            }
        })
    });
}

fn bench_rank(c: &mut Criterion) {
    bench_one_rank(
        c,
        "rsdict::rank",
        |bits| {
            let mut rs_dict = RsDict::with_capacity(NUM_BITS);
            for b in bits.iter() {
                rs_dict.push(b);
            }
            rs_dict
        },
        |r, i| r.rank(i, true)
    );
    bench_one_rank(
        c,
        "jacobson::rank",
        JacobsonRank::new,
        |r, i| r.rank(i, true)
    );
    bench_one_rank(
        c,
        "rank9::rank",
        Rank9::new,
        |r, i| r.rank(i, true)
    );
}

fn bench_one_select<T, F, G, H>(c: &mut Criterion, name: &str, create: F, select0: G, select1: H)
where
    F: Fn(BitVector<u64>) -> T,
    G: Fn(&T, u64) -> Option<u64>,
    H: Fn(&T, u64) -> Option<u64>
{
    let bits = random_bits(NUM_BITS);
    let num_set = bits.iter().filter(|&b| b).count();
    let r = create(bits);
    let indices = random_indices(1000, num_set);

    c.bench_function(&format!("{}::select0", name), |b| {
        b.iter(|| {
            for &ix in &indices {
                select0(&r, black_box(ix as u64));
            }
        })
    });
    c.bench_function(&format!("{}::select1", name), |b| {
        b.iter(|| {
            for &ix in &indices {
                select1(&r, black_box(ix as u64));
            }
        })
    });
}

fn bench_select(c: &mut Criterion) {
    bench_one_select(
        c,
        "rsdict",
        |bits| {
            let mut rs_dict = RsDict::with_capacity(NUM_BITS);
            for b in bits.iter() {
                rs_dict.push(b);
            }
            rs_dict
        },
        |r, i| r.select0(i),
        |r, i| r.select1(i),
    );
    bench_one_select(
        c,
        "rank9::binsearch",
        |b| BinSearchSelect::new(Rank9::new(b)),
        |r, i| r.select0(i),
        |r, i| r.select1(i),
    );
}

use rand::rngs::SmallRng;
use rand::RngCore;

pub const SEEDS: [u8; 16] = [
    0xde, 0xad, 0xbe, 0xef, 0xc0, 0xfe, 0xbe, 0xbe, 0xde, 0xad, 0xbe, 0xef, 0xc0, 0xfe, 0xbe, 0xbe,
];

/// Test that everything runs properly in the PPI graph.
pub fn build_random_sorted_vector(size: usize, max: u64) -> Vec<u64> {
    let mut rng: SmallRng = SmallRng::from_seed(SEEDS);
    let mut vector = Vec::new();
    for _ in 0..size {
        let t = rng.next_u64() % max;
        vector.push(t);
    }
    vector.sort();
    vector
}

fn bench_iter(c: &mut Criterion) {

    // crete a random vector of values
    let vector = build_random_sorted_vector(1000, 10_000_000);
    
    let mut r = rsdict::RsDict::new();
    // initialize the struct with its values
    let mut last_v = 0;
    for v in &vector{
        for _ in 0..(v - last_v).saturating_sub(1) {
            r.push(false);
        }
        r.push(true);
        last_v = *v;
    }
    c.bench_function("bench_iter", |b| {
        b.iter(|| {
            r.iter().collect::<Vec<_>>()
        });
    });

    c.bench_function("bench_iter_with_select", |b| {
        b.iter(|| {
            (0..r.len()).map(|i| r.select1(i as u64)).collect::<Vec<_>>()
        });
    });
}

criterion_group!(benches, bench_iter); //, bench_rank, bench_select);
criterion_main!(benches);
