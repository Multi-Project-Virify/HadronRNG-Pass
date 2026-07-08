//! Tests for the HadronRNG generator.

use hadronrng::HadronRng;

const PASSWORD_LEN: usize = 32;

#[test]
fn generator_creation() {
    let rng = HadronRng::new();

    assert!(rng.is_ok());
}

#[test]
fn random_bytes_length() {
    let mut rng = HadronRng::new()
        .expect("failed to create generator");

    let bytes = rng.random_bytes(64)
        .expect("failed to generate random bytes");

    assert_eq!(bytes.len(), 64);
}

#[test]
fn random_bytes_are_not_equal() {
    let mut rng = HadronRng::new()
        .expect("failed to create generator");

    let first = rng.random_bytes(64)
        .expect("failed to generate bytes");

    let second = rng.random_bytes(64)
        .expect("failed to generate bytes");

    assert_ne!(first, second);
}

#[test]
fn password_generation() {
    let mut rng = HadronRng::new()
        .expect("failed to create generator");

    let password = rng.password(PASSWORD_LEN)
        .expect("failed to generate password");

    assert_eq!(password.len(), PASSWORD_LEN);
}

#[test]
fn generated_passwords_should_be_different() {
    let mut rng = HadronRng::new()
        .expect("failed to create generator");

    let first = rng.password(PASSWORD_LEN)
        .expect("failed to generate password");

    let second = rng.password(PASSWORD_LEN)
        .expect("failed to generate password");

    assert_ne!(first, second);
}

#[test]
fn generate_key_128() {
    let mut rng = HadronRng::new()
        .expect("failed to create generator");

    let key = rng.key_128()
        .expect("failed to generate key");

    assert_eq!(key.len(), 16);
}

#[test]
fn generate_key_256() {
    let mut rng = HadronRng::new()
        .expect("failed to create generator");

    let key = rng.key_256()
        .expect("failed to generate key");

    assert_eq!(key.len(), 32);
}

#[test]
fn generate_key_512() {
    let mut rng = HadronRng::new()
        .expect("failed to create generator");

    let key = rng.key_512()
        .expect("failed to generate key");

    assert_eq!(key.len(), 64);
}

#[test]
fn zero_length_password_should_fail() {
    let mut rng = HadronRng::new()
        .expect("failed to create generator");

    assert!(rng.password(0).is_err());
}

#[test]
fn zero_length_random_bytes_should_return_empty() {
    let mut rng = HadronRng::new()
        .expect("failed to create generator");

    let bytes = rng.random_bytes(0)
        .expect("failed to generate bytes");

    assert!(bytes.is_empty());
}

#[test]
fn large_random_buffer() {
    let mut rng = HadronRng::new()
        .expect("failed to create generator");

    let bytes = rng.random_bytes(1024 * 1024)
        .expect("failed to generate bytes");

    assert_eq!(bytes.len(), 1024 * 1024);
}

#[test]
fn multiple_generations() {
    let mut rng = HadronRng::new()
        .expect("failed to create generator");

    for _ in 0..10_000 {
        let password = rng.password(16)
            .expect("failed to generate password");

        assert_eq!(password.len(), 16);
    }
}