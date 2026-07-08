//! Tests for password generation.

use std::collections::HashSet;

use hadronrng::HadronRng;

const DEFAULT_LEN: usize = 32;

#[test]
fn password_has_requested_length() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let password = rng.password(DEFAULT_LEN)
        .expect("failed to generate password");

    assert_eq!(password.len(), DEFAULT_LEN);
}

#[test]
fn generated_passwords_are_unique() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let p1 = rng.password(DEFAULT_LEN)
        .expect("failed to generate first password");

    let p2 = rng.password(DEFAULT_LEN)
        .expect("failed to generate second password");

    assert_ne!(p1, p2);
}

#[test]
fn generate_many_unique_passwords() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let mut passwords = HashSet::new();

    for _ in 0..1000 {
        let password = rng.password(DEFAULT_LEN)
            .expect("failed to generate password");

        assert!(
            passwords.insert(password),
            "duplicate password generated"
        );
    }
}

#[test]
fn minimum_password_length() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let password = rng.password(1)
        .expect("failed to generate password");

    assert_eq!(password.len(), 1);
}

#[test]
fn zero_length_password_returns_error() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    assert!(rng.password(0).is_err());
}

#[test]
fn large_password_generation() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let password = rng.password(4096)
        .expect("failed to generate password");

    assert_eq!(password.len(), 4096);
}

#[test]
fn passwords_are_not_empty() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    for _ in 0..100 {
        let password = rng.password(DEFAULT_LEN)
            .expect("failed to generate password");

        assert!(!password.is_empty());
    }
}

#[test]
fn passwords_contain_more_than_one_unique_character() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let password = rng.password(128)
        .expect("failed to generate password");

    let unique: HashSet<char> = password.chars().collect();

    assert!(
        unique.len() > 1,
        "generated password contains only one unique character"
    );
}

#[test]
fn generate_passwords_multiple_times() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    for _ in 0..10_000 {
        let password = rng.password(16)
            .expect("failed to generate password");

        assert_eq!(password.len(), 16);
    }
}