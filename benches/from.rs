use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct u24([u8; 3]);

fn u322u24(x: u32) -> u24 {
    let [a, b, c, _] = x.to_le_bytes();
    u24([a, b, c])
}

fn u642u24(x: u64) -> u24 {
    let [a, b, c, _, _, _, _, _] = x.to_le_bytes();
    u24([a, b, c])
}

fn from2u32_by_to_le_bytes(n: u32) {
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        let x: u32 = rng.gen();
        let y: u24 = u322u24(x);
        let z = u32::from_le_bytes([y.0[0], y.0[1], y.0[2], 0]);
        std::hint::black_box(z);
    }
}

fn from2u32_by_shift(n: u32) {
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        let x: u32 = rng.gen();
        let y: u24 = u322u24(x);
        let z = (y.0[0] as u32) | ((y.0[1] as u32) << 8) | ((y.0[2] as u32) << 16);
        std::hint::black_box(z);
    }
}

fn from2u64_by_to_le_bytes(n: u32) {
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        let x: u64 = rng.gen();
        let y: u24 = u642u24(x);
        let z = u64::from_le_bytes([y.0[0], y.0[1], y.0[2], 0, 0, 0, 0, 0]);
        std::hint::black_box(z);
    }
}

fn from2u64_by_shift(n: u32) {
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        let x: u64 = rng.gen();
        let y: u24 = u642u24(x);
        let z = (y.0[0] as u64) | ((y.0[1] as u64) << 8) | ((y.0[2] as u64) << 16);
        std::hint::black_box(z);
    }
}

fn bench_from(c: &mut Criterion) {
    c.bench_function("from2u32_by_to_le_bytes", |b| {
        b.iter(|| from2u32_by_to_le_bytes(10000))
    });
    c.bench_function("from2u32_by_shift", |b| b.iter(|| from2u32_by_shift(10000)));
    c.bench_function("from2u64_by_to_le_bytes", |b| {
        b.iter(|| from2u64_by_to_le_bytes(10000))
    });
    c.bench_function("from2u64_by_shift", |b| b.iter(|| from2u64_by_shift(10000)));
}

criterion_group!(benches, bench_from);
criterion_main!(benches);
