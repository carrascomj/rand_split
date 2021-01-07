use criterion::{criterion_group, criterion_main, Criterion};

extern crate rand_split;

use rand::prelude::*;
use rand_split::split_parts;
use rand_split::TTVSplit;

fn split_slice_big(rng: &mut ThreadRng) {
    let mut cont: Vec<usize> = (0..1000000).collect();
    cont.shuffle(rng);
    let result = split_parts(&mut cont, &[1.5, 2.5, 0.2, 12.0, 4.5]);
    assert!(result[0].len() < result[3].len());
}
fn split_iterator_big(rng: &mut ThreadRng) {
    let mut cont: Vec<usize> = (0..10000).collect();
    cont.shuffle(rng);
    let _result = cont
        .iter()
        .split_ttv([3., 8., 2.])
        .map(|v| v.iter().filter(|n| n.is_some()).count())
        .collect::<Vec<usize>>();
}

fn split_slice_bench(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    c.bench_function("Split random slice n=10e7, 5 partitions", |b| {
        b.iter(|| split_slice_big(&mut rng))
    });
}

fn split_iterator_bench(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    c.bench_function("Split random iterator n=10e5, 3 partitions", |b| {
        b.iter(|| split_iterator_big(&mut rng))
    });
}

criterion_group!(benches, split_slice_bench, split_iterator_bench);
criterion_main!(benches);
