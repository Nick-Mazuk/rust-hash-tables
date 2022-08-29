use criterion::{criterion_group, criterion_main, Criterion};
use rand::prelude::*;
use rand::thread_rng as rng;
use std::collections::HashSet;

struct Data<T> {
    set: HashSet<T>,
    vec: Vec<T>,
}

fn create_number_data(n: usize) -> Data<usize> {
    let mut set = HashSet::new();
    let mut vec = Vec::new();
    for i in 0..n {
        set.insert(i);
        vec.push(i);
    }
    Data { set, vec }
}

fn create_string_data(n: usize) -> Data<String> {
    let mut set = HashSet::new();
    let mut vec = Vec::new();
    for i in 0..n {
        set.insert(format!("{}", i));
        vec.push(format!("{}", i));
    }
    Data { set, vec }
}

fn random(size: f64) -> usize {
    (rng().gen::<f64>() * size) as usize
}

fn criterion_benchmark(c: &mut Criterion) {
    let size = 100000;
    let number_data = create_number_data(size);
    let string_data = create_string_data(size);
    let size_float = size as f64;

    c.bench_function("number vec", |b| {
        b.iter(|| {
            number_data.vec.contains(&(random(size_float)));
        })
    });
    c.bench_function("number set", |b| {
        b.iter(|| {
            number_data.set.contains(&(random(size_float)));
        })
    });
    c.bench_function("string vec", |b| {
        b.iter(|| {
            string_data.vec.contains(&format!("{}", random(size_float)));
        })
    });
    c.bench_function("string set", |b| {
        b.iter(|| {
            string_data.set.contains(&format!("{}", random(size_float)));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
