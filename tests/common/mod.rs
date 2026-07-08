//! Shared test utilities for HadronRNG.
//!
//! These helpers are only used by integration tests.

#![allow(dead_code)]

use hadronrng::HadronRng;

/// Default password length used in tests.
pub const DEFAULT_PASSWORD_LENGTH: usize = 32;

/// Creates a new RNG instance for testing.
pub fn create_rng() -> HadronRng {
    HadronRng::new().expect("failed to create HadronRng")
}

/// Returns a password generated with the default settings.
pub fn generate_password() -> String {
    let mut rng = create_rng();

    rng.password(DEFAULT_PASSWORD_LENGTH)
        .expect("failed to generate password")
}

/// Checks that every character belongs to the allowed charset.
pub fn password_matches_charset(password: &str, charset: &str) -> bool {
    password.chars().all(|c| charset.contains(c))
}

/// Checks that the password length is correct.
pub fn password_has_length(password: &str, expected: usize) -> bool {
    password.chars().count() == expected
}

/// Returns true if every password is unique.
pub fn all_unique(passwords: &[String]) -> bool {
    use std::collections::HashSet;

    let mut set = HashSet::new();

    passwords.iter().all(|p| set.insert(p))
}

/// Generates multiple passwords.
pub fn generate_passwords(count: usize, length: usize) -> Vec<String> {
    let mut rng = create_rng();

    let mut passwords = Vec::with_capacity(count);

    for _ in 0..count {
        passwords.push(
            rng.password(length)
                .expect("password generation failed"),
        );
    }

    passwords
}

/// Generates random bytes.
pub fn generate_bytes(len: usize) -> Vec<u8> {
    let mut rng = create_rng();

    rng.random_bytes(len)
        .expect("random byte generation failed")
}

/// Generates a 256-bit key.
pub fn generate_key256() -> Vec<u8> {
    let mut rng = create_rng();

    rng.key_256()
        .expect("key generation failed")
}