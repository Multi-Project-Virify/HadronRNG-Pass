//! Tests for cryptographic key generation.

use std::collections::HashSet;

use hadronrng::HadronRng;

#[test]
fn generate_128_bit_key() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let key = rng.key_128()
        .expect("failed to generate 128-bit key");

    assert_eq!(key.len(), 16);
}

#[test]
fn generate_192_bit_key() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let key = rng.key_192()
        .expect("failed to generate 192-bit key");

    assert_eq!(key.len(), 24);
}

#[test]
fn generate_256_bit_key() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let key = rng.key_256()
        .expect("failed to generate 256-bit key");

    assert_eq!(key.len(), 32);
}

#[test]
fn generate_512_bit_key() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let key = rng.key_512()
        .expect("failed to generate 512-bit key");

    assert_eq!(key.len(), 64);
}

#[test]
fn generated_keys_are_unique() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let first = rng.key_256()
        .expect("failed to generate first key");

    let second = rng.key_256()
        .expect("failed to generate second key");

    assert_ne!(first, second);
}

#[test]
fn generate_many_unique_keys() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let mut keys = HashSet::new();

    for _ in 0..1000 {
        let key = rng.key_256()
            .expect("failed to generate key");

        assert!(
            keys.insert(key),
            "duplicate key generated"
        );
    }
}

#[test]
fn generated_key_is_not_all_zeroes() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let key = rng.key_256()
        .expect("failed to generate key");

    assert!(
        key.iter().any(|&b| b != 0),
        "generated key contains only zero bytes"
    );
}

#[test]
fn generate_large_number_of_keys() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    for _ in 0..10_000 {
        let key = rng.key_256()
            .expect("failed to generate key");

        assert_eq!(key.len(), 32);
    }
}

#[test]
fn keys_have_correct_lengths() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    assert_eq!(
        rng.key_128()
            .expect("failed to generate key")
            .len(),
        16
    );

    assert_eq!(
        rng.key_192()
            .expect("failed to generate key")
            .len(),
        24
    );

    assert_eq!(
        rng.key_256()
            .expect("failed to generate key")
            .len(),
        32
    );

    assert_eq!(
        rng.key_512()
            .expect("failed to generate key")
            .len(),
        64
    );
}

#[test]
fn generated_keys_should_not_repeat() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let mut previous = rng.key_256()
        .expect("failed to generate key");

    for _ in 0..100 {
        let current = rng.key_256()
            .expect("failed to generate key");

        assert_ne!(previous, current);

        previous = current;
    }
}