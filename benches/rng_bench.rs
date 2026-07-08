use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hadronrng::HadronRng;


fn random_bytes_benchmark(c: &mut Criterion) {

    c.bench_function(
        "random_bytes_1024",
        |b| {

            let mut rng =
                HadronRng::new()
                    .unwrap();


            b.iter(|| {

                let data =
                    rng.random_bytes(1024)
                        .unwrap();


                black_box(data);
            });
        }
    );
}


criterion_group!(
    benches,
    random_bytes_benchmark
);

criterion_main!(benches);