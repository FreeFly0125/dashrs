use criterion::{criterion_group, criterion_main, Criterion};
use dash_rs::{model::level::Level, HasRobtopFormat, Thunk};
use flate2::read::GzDecoder;
use std::{fs::read_to_string, io::Read};

pub fn ocular_miracle_benchmark(c: &mut Criterion) {
    let response = read_to_string("./benches/data/62152040_ocular_miracle_gjdownload_response").unwrap();

    c.bench_function("parse ocular machine", |b| {
        b.iter(|| {
            let level = Level::from_robtop_str(&response).unwrap();

            level.level_data.unwrap().level_data.process().unwrap();
        })
    });
}

pub fn spacial_rend_benchmark(c: &mut Criterion) {
    let response = read_to_string("./benches/data/40292513_special_rend_gjdownload_response").unwrap();

    c.bench_function("parse spacial rend", |b| {
        b.iter(|| {
            let level = Level::from_robtop_str(&response).unwrap();

            level.level_data.unwrap().level_data.process().unwrap();
        })
    });
}

pub fn decoding_ocular_miracle_benchmark(c: &mut Criterion) {
    let response = read_to_string("./benches/data/62152040_ocular_miracle_gjdownload_response").unwrap();

    c.bench_function("decode ocular miracle", |b| {
        b.iter(|| {
            let level = Level::from_robtop_str(&response).unwrap();
            match level.level_data.unwrap().level_data {
                Thunk::Unprocessed(unprocessed) => {
                    let decoded = base64::decode_config(unprocessed, base64::URL_SAFE).unwrap();
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
            let level = Level::from_robtop_str(&response).unwrap();
            match level.level_data.unwrap().level_data {
                Thunk::Unprocessed(unprocessed) => {
                    let decoded = base64::decode_config(unprocessed, base64::URL_SAFE).unwrap();
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
