//! Tests for password generation policies.

use hadronrng::{HadronRng, PasswordPolicy};

#[test]
fn default_policy_is_valid() {
    let policy = PasswordPolicy::default();

    assert!(policy.validate().is_ok());
}

#[test]
fn generate_password_using_default_policy() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let policy = PasswordPolicy::default();

    let password = rng
        .password_with_policy(&policy)
        .expect("failed to generate password");

    assert_eq!(password.len(), policy.length());
}

#[test]
fn custom_length_policy() {
    let policy = PasswordPolicy::builder()
        .length(64)
        .build()
        .expect("failed to build policy");

    assert_eq!(policy.length(), 64);
}

#[test]
fn zero_length_policy_is_invalid() {
    let result = PasswordPolicy::builder()
        .length(0)
        .build();

    assert!(result.is_err());
}

#[test]
fn one_character_password_is_allowed() {
    let policy = PasswordPolicy::builder()
        .length(1)
        .build()
        .expect("failed to build policy");

    assert_eq!(policy.length(), 1);
}

#[test]
fn very_large_password_length() {
    let policy = PasswordPolicy::builder()
        .length(4096)
        .build()
        .expect("failed to build policy");

    assert_eq!(policy.length(), 4096);
}

#[test]
fn generate_password_with_custom_policy() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let policy = PasswordPolicy::builder()
        .length(128)
        .build()
        .expect("failed to build policy");

    let password = rng
        .password_with_policy(&policy)
        .expect("failed to generate password");

    assert_eq!(password.len(), 128);
}

#[test]
fn generated_passwords_are_unique() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let policy = PasswordPolicy::default();

    let p1 = rng
        .password_with_policy(&policy)
        .expect("failed to generate first password");

    let p2 = rng
        .password_with_policy(&policy)
        .expect("failed to generate second password");

    assert_ne!(p1, p2);
}

#[test]
fn policy_can_be_reused() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let policy = PasswordPolicy::builder()
        .length(32)
        .build()
        .expect("failed to build policy");

    for _ in 0..100 {
        let password = rng
            .password_with_policy(&policy)
            .expect("failed to generate password");

        assert_eq!(password.len(), 32);
    }
}

#[test]
fn cloned_policy_behaves_identically() {
    let policy = PasswordPolicy::default();

    let cloned = policy.clone();

    assert_eq!(policy.length(), cloned.length());
}