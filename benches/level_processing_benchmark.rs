use base64::{engine::general_purpose::URL_SAFE, Engine};
use criterion::{criterion_group, criterion_main, Criterion};
use dash_rs::{
    model::level::{Level, LevelData},
    GJFormat, Thunk,
};
use flate2::read::GzDecoder;
use std::{fs::read_to_string, io::Read};

pub fn ocular_miracle_benchmark(c: &mut Criterion) {
    let response = read_to_string("./benches/data/62152040_ocular_miracle_gjdownload_response").unwrap();

    c.bench_function("parse ocular machine", |b| {
        b.iter(|| {
            let mut level: Level<LevelData> = Level::from_gj_str(&response).unwrap();

            level.level_data.level_data.process().unwrap();
        })
    });
}

pub fn spacial_rend_benchmark(c: &mut Criterion) {
    let response = read_to_string("./benches/data/40292513_special_rend_gjdownload_response").unwrap();

    c.bench_function("parse spacial rend", |b| {
        b.iter(|| {
            let mut level: Level<LevelData> = Level::from_gj_str(&response).unwrap();

            level.level_data.level_data.process().unwrap();
        })
    });
}

pub fn decoding_ocular_miracle_benchmark(c: &mut Criterion) {
    let response = read_to_string("./benches/data/62152040_ocular_miracle_gjdownload_response").unwrap();

    c.bench_function("decode ocular miracle", |b| {
        b.iter(|| {
            let level: Level<LevelData> = Level::from_gj_str(&response).unwrap();
            match level.level_data.level_data {
                Thunk::Unprocessed(unprocessed) => {
                    let decoded = URL_SAFE.decode(&*unprocessed).unwrap();
                    let mut decompressed = String::new();
                    let mut decoder = GzDecoder::new(&decoded[..]);

                    decoder.read_to_string(&mut decompressed).unwrap();
                },
                Thunk::Processed(_) => unreachable!(),
            }
        })
    });
}

pub fn decoding_spacial_rend_benchmark(c: &mut Criterion) {
    let response = read_to_string("./benches/data/40292513_special_rend_gjdownload_response").unwrap();

    c.bench_function("decode spacial rend", |b| {
        b.iter(|| {
            let level: Level<LevelData> = Level::from_gj_str(&response).unwrap();
            match level.level_data.level_data {
                Thunk::Unprocessed(unprocessed) => {
                    let decoded = URL_SAFE.decode(&*unprocessed).unwrap();
                    let mut decompressed = String::new();
                    let mut decoder = GzDecoder::new(&decoded[..]);

                    decoder.read_to_string(&mut decompressed).unwrap();
                },
                Thunk::Processed(_) => unreachable!(),
            }
        })
    });
}

criterion_group!(
    benches,
    ocular_miracle_benchmark,
    spacial_rend_benchmark,
    decoding_spacial_rend_benchmark,
    decoding_ocular_miracle_benchmark
);
criterion_main!(benches);
