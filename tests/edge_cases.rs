//! Edge case tests for HadronRNG.

use hadronrng::{HadronRng, PasswordPolicy};

#[test]
fn password_length_one() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let password = rng.password(1)
        .expect("failed to generate password");

    assert_eq!(password.len(), 1);
}

#[test]
fn password_length_zero_returns_error() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    assert!(rng.password(0).is_err());
}

#[test]
fn generate_very_large_password() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let password = rng.password(100_000)
        .expect("failed to generate password");

    assert_eq!(password.len(), 100_000);
}

#[test]
fn generate_zero_random_bytes() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let bytes = rng.random_bytes(0)
        .expect("failed to generate bytes");

    assert!(bytes.is_empty());
}

#[test]
fn generate_large_random_buffer() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let bytes = rng.random_bytes(10 * 1024 * 1024)
        .expect("failed to generate bytes");

    assert_eq!(bytes.len(), 10 * 1024 * 1024);
}

#[test]
fn key_zero_bits_returns_error() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    assert!(rng.key(0).is_err());
}

#[test]
fn key_non_byte_aligned_returns_error() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    assert!(rng.key(257).is_err());
}

#[test]
fn key_8192_bits() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let key = rng.key(8192)
        .expect("failed to generate key");

    assert_eq!(key.len(), 1024);
}

#[test]
fn empty_charset_returns_error() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    assert!(rng.password_with_charset(32, "").is_err());
}

#[test]
fn single_character_charset() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let password = rng.password_with_charset(64, "A")
        .expect("failed to generate password");

    assert_eq!(password, "A".repeat(64));
}

#[test]
fn duplicate_charset_is_allowed() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let password = rng.password_with_charset(64, "AAAABBBBCCCC")
        .expect("failed to generate password");

    assert_eq!(password.len(), 64);
}

#[test]
fn policy_zero_length_fails() {
    assert!(
        PasswordPolicy::builder()
            .length(0)
            .build()
            .is_err()
    );
}

#[test]
fn policy_large_length() {
    let policy = PasswordPolicy::builder()
        .length(65_535)
        .build()
        .expect("failed to build policy");

    assert_eq!(policy.length(), 65_535);
}

#[test]
fn many_small_passwords() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    for _ in 0..100_000 {
        let password = rng.password(1)
            .expect("failed to generate password");

        assert_eq!(password.len(), 1);
    }
}

#[test]
fn many_large_passwords() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    for _ in 0..100 {
        let password = rng.password(4096)
            .expect("failed to generate password");

        assert_eq!(password.len(), 4096);
    }
}