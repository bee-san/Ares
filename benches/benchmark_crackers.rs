use ares::decoders::base64_decoder::Base64Decoder;
use ares::decoders::interface::{Crack, Decoder};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let decode_base64 = Decoder::<Base64Decoder>::new();
    c.bench_function("base64 successful decoding", |b| {
        b.iter(|| decode_base64.crack(black_box("aGVsbG8gd29ybGQ=")))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
