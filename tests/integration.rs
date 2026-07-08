//! Integration tests for HadronRNG.

use hadronrng::{HadronRng, PasswordPolicy};

#[test]
fn complete_password_workflow() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let password = rng.password(32)
        .expect("failed to generate password");

    assert_eq!(password.len(), 32);
}

#[test]
fn complete_key_workflow() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let key = rng.key(256)
        .expect("failed to generate key");

    assert_eq!(key.len(), 32);
}

#[test]
fn random_bytes_workflow() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let bytes = rng.random_bytes(512)
        .expect("failed to generate random bytes");

    assert_eq!(bytes.len(), 512);
}

#[test]
fn policy_workflow() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let policy = PasswordPolicy::builder()
        .length(64)
        .build()
        .expect("failed to build policy");

    let password = rng.password_with_policy(&policy)
        .expect("failed to generate password");

    assert_eq!(password.len(), 64);
}

#[test]
fn custom_charset_workflow() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let charset = "abcdef123456";

    let password = rng.password_with_charset(128, charset)
        .expect("failed to generate password");

    assert_eq!(password.len(), 128);

    assert!(password.chars().all(|c| charset.contains(c)));
}

#[test]
fn password_and_key_are_independent() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let password = rng.password(64)
        .expect("failed to generate password");

    let key = rng.key(512)
        .expect("failed to generate key");

    assert_eq!(password.len(), 64);
    assert_eq!(key.len(), 64);
}

#[test]
fn multiple_operations() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    for _ in 0..1000 {
        let _ = rng.password(32)
            .expect("password");

        let _ = rng.key(256)
            .expect("key");

        let _ = rng.random_bytes(128)
            .expect("bytes");
    }
}

#[test]
fn independent_instances() {
    let mut rng1 = HadronRng::new()
        .expect("failed to create rng1");

    let mut rng2 = HadronRng::new()
        .expect("failed to create rng2");

    let p1 = rng1.password(32)
        .expect("password");

    let p2 = rng2.password(32)
        .expect("password");

    assert_ne!(p1, p2);
}

#[test]
fn generated_keys_are_unique() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let k1 = rng.key(256)
        .expect("key");

    let k2 = rng.key(256)
        .expect("key");

    assert_ne!(k1, k2);
}

#[test]
fn generated_random_buffers_are_unique() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let b1 = rng.random_bytes(1024)
        .expect("bytes");

    let b2 = rng.random_bytes(1024)
        .expect("bytes");

    assert_ne!(b1, b2);
}