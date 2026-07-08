//! Tests for character sets.

use std::collections::HashSet;

use hadronrng::{Charset, HadronRng};

#[test]
fn lowercase_charset_is_not_empty() {
    assert!(!Charset::LOWERCASE.is_empty());
}

#[test]
fn uppercase_charset_is_not_empty() {
    assert!(!Charset::UPPERCASE.is_empty());
}

#[test]
fn digits_charset_is_not_empty() {
    assert!(!Charset::DIGITS.is_empty());
}

#[test]
fn symbols_charset_is_not_empty() {
    assert!(!Charset::SYMBOLS.is_empty());
}

#[test]
fn default_charset_is_not_empty() {
    assert!(!Charset::DEFAULT.is_empty());
}

#[test]
fn default_charset_has_unique_characters() {
    let mut set = HashSet::new();

    for c in Charset::DEFAULT.chars() {
        assert!(
            set.insert(c),
            "duplicate character '{c}' found in DEFAULT charset"
        );
    }
}

#[test]
fn generate_password_with_default_charset() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let password = rng
        .password(64)
        .expect("failed to generate password");

    for c in password.chars() {
        assert!(
            Charset::DEFAULT.contains(c),
            "character '{c}' is not part of DEFAULT charset"
        );
    }
}

#[test]
fn generate_password_with_custom_charset() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let charset = "ABC123";

    let password = rng
        .password_with_charset(128, charset)
        .expect("failed to generate password");

    for c in password.chars() {
        assert!(
            charset.contains(c),
            "character '{c}' is not allowed"
        );
    }
}

#[test]
fn empty_charset_returns_error() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    assert!(
        rng.password_with_charset(32, "").is_err()
    );
}

#[test]
fn one_character_charset_is_supported() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let password = rng
        .password_with_charset(32, "X")
        .expect("failed to generate password");

    assert_eq!(password, "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
}

#[test]
fn custom_charset_produces_only_expected_characters() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let charset = "abcdef";

    for _ in 0..100 {
        let password = rng
            .password_with_charset(64, charset)
            .expect("failed to generate password");

        assert!(
            password.chars().all(|c| charset.contains(c))
        );
    }
}

#[test]
fn custom_charset_without_duplicates() {
    let charset = Charset::DEFAULT;

    let mut set = HashSet::new();

    for c in charset.chars() {
        assert!(
            set.insert(c),
            "duplicate character '{c}'"
        );
    }
}