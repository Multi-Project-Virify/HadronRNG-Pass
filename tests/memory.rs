//! Tests for secure memory handling.

use hadronrng::HadronRng;

#[test]
fn generated_password_can_be_cleared() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let mut password = rng.password(32)
        .expect("failed to generate password");

    assert!(!password.is_empty());

    hadronrng::memory::clear_string(&mut password);

    assert!(password.is_empty());
}

#[test]
fn generated_key_can_be_cleared() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let mut key = rng.key(256)
        .expect("failed to generate key");

    assert!(key.iter().any(|&b| b != 0));

    hadronrng::memory::clear_bytes(&mut key);

    assert!(key.iter().all(|&b| b == 0));
}

#[test]
fn clear_empty_string() {
    let mut value = String::new();

    hadronrng::memory::clear_string(&mut value);

    assert!(value.is_empty());
}

#[test]
fn clear_empty_bytes() {
    let mut bytes = Vec::<u8>::new();

    hadronrng::memory::clear_bytes(&mut bytes);

    assert!(bytes.is_empty());
}

#[test]
fn clear_large_buffer() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let mut bytes = rng.random_bytes(1024 * 1024)
        .expect("failed to generate bytes");

    hadronrng::memory::clear_bytes(&mut bytes);

    assert!(bytes.iter().all(|&b| b == 0));
}

#[test]
fn multiple_clear_operations() {
    let mut bytes = vec![1u8; 256];

    hadronrng::memory::clear_bytes(&mut bytes);
    hadronrng::memory::clear_bytes(&mut bytes);
    hadronrng::memory::clear_bytes(&mut bytes);

    assert!(bytes.iter().all(|&b| b == 0));
}

#[test]
fn clear_password_multiple_times() {
    let mut password = String::from("VerySecretPassword");

    hadronrng::memory::clear_string(&mut password);
    hadronrng::memory::clear_string(&mut password);

    assert!(password.is_empty());
}

#[test]
fn clear_key_multiple_times() {
    let mut key = vec![0xAA; 64];

    hadronrng::memory::clear_bytes(&mut key);
    hadronrng::memory::clear_bytes(&mut key);

    assert!(key.iter().all(|&b| b == 0));
}

#[test]
fn clear_random_buffer() {
    let mut rng = HadronRng::new()
        .expect("failed to create HadronRng");

    let mut buffer = rng.random_bytes(4096)
        .expect("failed to generate random bytes");

    hadronrng::memory::clear_bytes(&mut buffer);

    assert!(buffer.iter().all(|&b| b == 0));
}

#[test]
fn clear_does_not_change_length() {
    let mut bytes = vec![0x55; 128];

    let len = bytes.len();

    hadronrng::memory::clear_bytes(&mut bytes);

    assert_eq!(bytes.len(), len);
}