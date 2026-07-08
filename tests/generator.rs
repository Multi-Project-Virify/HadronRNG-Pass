use hadronrng::HadronRng;


#[test]
fn generator_creates_instance() {

    let rng =
        HadronRng::new();


    assert!(
        rng.is_ok(),
        "HadronRng initialization failed"
    );
}



#[test]
fn generator_generates_bytes() {

    let mut rng =
        HadronRng::new()
            .expect("failed to create rng");


    let data =
        rng.random_bytes(128)
            .expect("failed to generate bytes");


    assert_eq!(
        data.len(),
        128
    );
}



#[test]
fn generator_generates_small_data() {

    let mut rng =
        HadronRng::new()
            .expect("failed");


    let data =
        rng.random_bytes(1)
            .expect("failed");


    assert_eq!(
        data.len(),
        1
    );
}



#[test]
fn generator_generates_large_data() {

    let mut rng =
        HadronRng::new()
            .expect("failed");


    let data =
        rng.random_bytes(1024 * 1024)
            .expect("failed");


    assert_eq!(
        data.len(),
        1024 * 1024
    );
}



#[test]
fn generator_multiple_operations() {

    let mut rng =
        HadronRng::new()
            .expect("failed");


    for _ in 0..10_000 {

        let data =
            rng.random_bytes(64)
                .expect("generation failed");


        assert_eq!(
            data.len(),
            64
        );
    }
}



#[test]
fn generator_outputs_are_unique() {

    let mut rng =
        HadronRng::new()
            .expect("failed");


    let first =
        rng.random_bytes(64)
            .expect("failed");


    let second =
        rng.random_bytes(64)
            .expect("failed");


    assert_ne!(
        first,
        second,
        "generator produced identical output"
    );
}



#[test]
fn generator_multiple_instances() {

    let mut rng1 =
        HadronRng::new()
            .expect("failed");


    let mut rng2 =
        HadronRng::new()
            .expect("failed");


    let data1 =
        rng1.random_bytes(64)
            .expect("failed");


    let data2 =
        rng2.random_bytes(64)
            .expect("failed");


    assert_ne!(
        data1,
        data2,
        "different RNG instances produced same data"
    );
}



#[test]
fn generator_stability_test() {

    let mut rng =
        HadronRng::new()
            .expect("failed");


    for size in [
        16usize,
        32,
        64,
        128,
        256,
        512,
    ] {

        let data =
            rng.random_bytes(size)
                .expect("generation failed");


        assert_eq!(
            data.len(),
            size
        );
    }
}