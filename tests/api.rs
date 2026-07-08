//! Public API tests for HadronRNG.

use hadronrng::HadronRng;

#[test]
fn create_rng() {
    let rng = HadronRng::new();

    assert!(rng.is_ok());
}

#[test]
fn generate_default_password() {
    let mut rng = HadronRng::new().unwrap();

    let password = rng.password(32).unwrap();

    assert_eq!(password.len(), 32);
}

#[test]
fn generate_random_bytes() {
    let mut rng = HadronRng::new().unwrap();

    let bytes = rng.random_bytes(64).unwrap();

    assert_eq!(bytes.len(), 64);
}

#[test]
fn generate_key_128() {
    let mut rng = HadronRng::new().unwrap();

    let key = rng.key_128().unwrap();

    assert_eq!(key.len(), 16);
}

#[test]
fn generate_key_256() {
    let mut rng = HadronRng::new().unwrap();

    let key = rng.key_256().unwrap();

    assert_eq!(key.len(), 32);
}

#[test]
fn generate_key_512() {
    let mut rng = HadronRng::new().unwrap();

    let key = rng.key_512().unwrap();

    assert_eq!(key.len(), 64);
}

#[test]
fn passwords_should_not_match() {
    let mut rng = HadronRng::new().unwrap();

    let p1 = rng.password(32).unwrap();
    let p2 = rng.password(32).unwrap();

    assert_ne!(p1, p2);
}

#[test]
fn random_bytes_should_not_match() {
    let mut rng = HadronRng::new().unwrap();

    let a = rng.random_bytes(64).unwrap();
    let b = rng.random_bytes(64).unwrap();

    assert_ne!(a, b);
}

#[test]
fn empty_password_length_should_fail() {
    let mut rng = HadronRng::new().unwrap();

    assert!(rng.password(0).is_err());
}

#[test]
fn large_password_generation() {
    let mut rng = HadronRng::new().unwrap();

    let password = rng.password(4096).unwrap();

    assert_eq!(password.len(), 4096);
}

#[test]
fn generate_many_passwords() {
    let mut rng = HadronRng::new().unwrap();

    for _ in 0..1000 {
        let password = rng.password(32).unwrap();
        assert_eq!(password.len(), 32);
    }
}