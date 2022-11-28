use std::fs::File;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rusttracer::util::outputbuffer::OutputBuffer;
use rusttracer::util::vector::Vector;

pub fn call_set_at() {
    let mut temp = OutputBuffer::with_size(
        10,
        10,
        "back.rgb",
    );
    let mut f = File::create("back.rgb").unwrap();

    for x in 0..10{
        for y in 0..10{
            let vec = Vector::new(x as f64, y as f64, (x + y)as f64);
            temp.set_at_basic(x, y, vec, &mut f);
        }
    }
}

pub fn set_at_benchmark(c: &mut Criterion){
    c.bench_function("benchmarking set_at", |b| {

        b.iter(|| {
            call_set_at()
        })
    });

}

criterion_group!(benches, set_at_benchmark);
criterion_main!(benches);