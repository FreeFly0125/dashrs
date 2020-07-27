use criterion::{criterion_group, criterion_main, Criterion};
use std::fs::read_to_string;
use dash_rs::model::level::Level;

pub fn ocular_miracle_benchmark(c: &mut Criterion) {
    let response = read_to_string("./benches/data/62152040_ocular_miracle_gjdownload_response").unwrap();

    c.bench_function("parse ocular machine", |b| b.iter(|| {
        let level = dash_rs::from_robtop_str::<Level<_, _>>(&response).unwrap();

        level.level_data.unwrap().level_data.process().unwrap();
    }));
}

pub fn spacial_rend_benchmark(c: &mut Criterion) {
    let response = read_to_string("./benches/data/40292513_special_rend_gjdownload_response").unwrap();

    c.bench_function("parse spacial rend", |b| b.iter(|| {
        let level = dash_rs::from_robtop_str::<Level<_, _>>(&response).unwrap();

        level.level_data.unwrap().level_data.process().unwrap();
    }));
}

criterion_group!(benches, ocular_miracle_benchmark, spacial_rend_benchmark);
criterion_main!(benches);