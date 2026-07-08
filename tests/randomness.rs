//! Statistical randomness tests.

use hadronrng::HadronRng;


const SAMPLE_SIZE: usize = 1_000_000;



#[test]
fn byte_frequency_test() {

    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");


    let data = rng.random_bytes(SAMPLE_SIZE)
        .expect("generation failed");


    let mut counters = [0usize; 256];


    for byte in data {

        counters[byte as usize] += 1;
    }


    let expected =
        SAMPLE_SIZE as f64 / 256.0;


    for count in counters {

        let diff =
            (count as f64 - expected).abs();


        assert!(
            diff < expected * 0.15,
            "bad byte distribution"
        );
    }
}



#[test]
fn bit_balance_test() {

    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");


    let data = rng.random_bytes(200_000)
        .expect("generation failed");


    let mut ones = 0usize;


    for byte in data {

        ones += byte.count_ones() as usize;
    }


    let total_bits = 200_000 * 8;


    let ratio =
        ones as f64 / total_bits as f64;


    assert!(
        ratio > 0.45 && ratio < 0.55,
        "bit distribution abnormal: {}",
        ratio
    );
}



#[test]
fn no_small_cycle_pattern() {

    let mut rng = HadronRng::new()
        .expect("failed");


    let a = rng.random_bytes(4096)
        .expect("failed");


    let b = rng.random_bytes(4096)
        .expect("failed");


    assert_ne!(
        a,
        b,
        "generator produced identical blocks"
    );
}