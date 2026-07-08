use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};

use hadronrng::HadronRng;



fn key_generation(c: &mut Criterion) {

    c.bench_function(
        "key_256",
        |b| {

            let mut rng =
                HadronRng::new()
                    .unwrap();


            b.iter(|| {

                let key =
                    rng.key(256)
                        .unwrap();


                black_box(key);
            });
        }
    );
}



criterion_group!(
    benches,
    key_generation
);


criterion_main!(benches);