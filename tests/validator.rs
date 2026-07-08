//! Tests for input validation.

use hadronrng::{HadronRng, PasswordPolicy};

#[test]
fn reject_zero_length_password() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    assert!(rng.password(0).is_err());
}

#[test]
fn accept_minimum_length_password() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let password = rng.password(1)
        .expect("failed to generate password");

    assert_eq!(password.len(), 1);
}

#[test]
fn reject_empty_charset() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    assert!(rng.password_with_charset(32, "").is_err());
}

#[test]
fn accept_single_character_charset() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let password = rng.password_with_charset(8, "X")
        .expect("failed to generate password");

    assert_eq!(password, "XXXXXXXX");
}

#[test]
fn reject_policy_with_zero_length() {
    let result = PasswordPolicy::builder()
        .length(0)
        .build();

    assert!(result.is_err());
}

#[test]
fn accept_valid_policy() {
    let policy = PasswordPolicy::builder()
        .length(32)
        .build();

    assert!(policy.is_ok());
}

#[test]
fn reject_invalid_key_size() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    assert!(rng.key(0).is_err());
}

#[test]
fn accept_standard_key_sizes() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    assert!(rng.key(128).is_ok());
    assert!(rng.key(192).is_ok());
    assert!(rng.key(256).is_ok());
    assert!(rng.key(512).is_ok());
}

#[test]
fn reject_non_byte_aligned_key_size() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    assert!(rng.key(129).is_err());
}

#[test]
fn validator_should_not_panic() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let _ = rng.password(0);
    let _ = rng.key(0);
    let _ = rng.password_with_charset(32, "");
}