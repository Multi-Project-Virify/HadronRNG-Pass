//! Regression tests for HadronRNG.
//!
//! These tests protect against previously fixed bugs.

use std::collections::HashSet;

use hadronrng::{HadronRng, PasswordPolicy};

#[test]
fn password_length_is_always_correct() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    for len in 1..256 {
        let password = rng.password(len)
            .expect("failed to generate password");

        assert_eq!(password.len(), len);
    }
}

#[test]
fn password_generation_never_returns_empty() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    for _ in 0..10_000 {
        let password = rng.password(32)
            .expect("failed to generate password");

        assert!(!password.is_empty());
    }
}

#[test]
fn generated_passwords_do_not_repeat() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let mut set = HashSet::new();

    for _ in 0..5000 {
        let password = rng.password(32)
            .expect("failed to generate password");

        assert!(set.insert(password));
    }
}

#[test]
fn generated_keys_do_not_repeat() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let mut set = HashSet::new();

    for _ in 0..5000 {
        let key = rng.key(256)
            .expect("failed to generate key");

        assert!(set.insert(key));
    }
}

#[test]
fn random_buffers_do_not_repeat() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let first = rng.random_bytes(4096)
        .expect("failed to generate bytes");

    let second = rng.random_bytes(4096)
        .expect("failed to generate bytes");

    assert_ne!(first, second);
}

#[test]
fn zero_length_password_is_rejected() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    assert!(rng.password(0).is_err());
}

#[test]
fn invalid_key_size_is_rejected() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    assert!(rng.key(7).is_err());
    assert!(rng.key(15).is_err());
    assert!(rng.key(257).is_err());
}

#[test]
fn empty_charset_is_rejected() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    assert!(rng.password_with_charset(16, "").is_err());
}

#[test]
fn password_policy_still_works() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let policy = PasswordPolicy::builder()
        .length(48)
        .build()
        .expect("failed to build policy");

    let password = rng.password_with_policy(&policy)
        .expect("failed to generate password");

    assert_eq!(password.len(), 48);
}

#[test]
fn rng_survives_many_operations() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    for _ in 0..50_000 {
        let _ = rng.password(16)
            .expect("password");

        let _ = rng.key(256)
            .expect("key");

        let _ = rng.random_bytes(64)
            .expect("bytes");
    }
}