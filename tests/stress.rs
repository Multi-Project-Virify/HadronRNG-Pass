//! Heavy stress tests for HadronRNG.

use std::collections::HashSet;
use hadronrng::HadronRng;


const PASSWORD_ITERATIONS: usize = 100_000;
const KEY_ITERATIONS: usize = 100_000;
const BYTE_ITERATIONS: usize = 10_000;


#[test]
fn stress_password_generation() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");


    let mut unique = HashSet::new();


    for _ in 0..PASSWORD_ITERATIONS {

        let password = rng.password(32)
            .expect("password generation failed");


        assert_eq!(password.len(), 32);


        unique.insert(password);
    }


    assert_eq!(
        unique.len(),
        PASSWORD_ITERATIONS,
        "duplicate passwords detected"
    );
}



#[test]
fn stress_key_generation() {

    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");


    let mut unique = HashSet::new();


    for _ in 0..KEY_ITERATIONS {

        let key = rng.key(256)
            .expect("key generation failed");


        assert_eq!(key.len(), 32);


        unique.insert(key);
    }


    assert_eq!(
        unique.len(),
        KEY_ITERATIONS,
        "duplicate keys detected"
    );
}



#[test]
fn stress_random_bytes_generation() {

    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");


    let mut total_bytes = 0usize;


    for _ in 0..BYTE_ITERATIONS {

        let bytes = rng.random_bytes(4096)
            .expect("byte generation failed");


        assert_eq!(
            bytes.len(),
            4096
        );


        total_bytes += bytes.len();
    }


    assert_eq!(
        total_bytes,
        BYTE_ITERATIONS * 4096
    );
}



#[test]
fn long_running_generation_cycle() {

    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");


    for _ in 0..1_000_000 {

        let data = rng.random_bytes(64)
            .expect("generator stopped");


        assert_eq!(
            data.len(),
            64
        );
    }
}