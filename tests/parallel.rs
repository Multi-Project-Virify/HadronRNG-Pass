//! Parallel tests for HadronRNG.

use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;

use hadronrng::HadronRng;

const THREADS: usize = 16;
const ITERATIONS: usize = 1000;

#[test]
fn parallel_password_generation() {
    let passwords = Arc::new(Mutex::new(Vec::<String>::new()));

    let mut handles = Vec::new();

    for _ in 0..THREADS {
        let passwords = Arc::clone(&passwords);

        handles.push(thread::spawn(move || {
            let mut rng = HadronRng::new()
                .expect("failed to create HadronRng");

            for _ in 0..ITERATIONS {
                let password = rng.password(32)
                    .expect("failed to generate password");

                passwords
                    .lock()
                    .expect("mutex poisoned")
                    .push(password);
            }
        }));
    }

    for handle in handles {
        handle.join()
            .expect("thread panicked");
    }

    let passwords = passwords
        .lock()
        .expect("mutex poisoned");

    assert_eq!(passwords.len(), THREADS * ITERATIONS);

    let unique: HashSet<String> = passwords
        .iter()
        .cloned()
        .collect();

    assert_eq!(unique.len(), passwords.len());
}


#[test]
fn parallel_key_generation() {
    let keys = Arc::new(Mutex::new(Vec::<Vec<u8>>::new()));

    let mut handles = Vec::new();

    for _ in 0..THREADS {
        let keys = Arc::clone(&keys);

        handles.push(thread::spawn(move || {
            let mut rng = HadronRng::new()
                .expect("failed to create HadronRng");

            for _ in 0..ITERATIONS {
                let key = rng.key(256)
                    .expect("failed to generate key");

                keys
                    .lock()
                    .expect("mutex poisoned")
                    .push(key);
            }
        }));
    }

    for handle in handles {
        handle.join()
            .expect("thread panicked");
    }

    let keys = keys
        .lock()
        .expect("mutex poisoned");

    assert_eq!(keys.len(), THREADS * ITERATIONS);

    let unique: HashSet<Vec<u8>> = keys
        .iter()
        .cloned()
        .collect();

    assert_eq!(unique.len(), keys.len());
}


#[test]
fn parallel_random_bytes() {
    let mut handles = Vec::new();

    for _ in 0..THREADS {
        handles.push(thread::spawn(|| {
            let mut rng = HadronRng::new()
                .expect("failed to create HadronRng");

            for _ in 0..ITERATIONS {
                let bytes = rng.random_bytes(1024)
                    .expect("failed to generate bytes");

                assert_eq!(bytes.len(), 1024);
            }
        }));
    }

    for handle in handles {
        handle.join()
            .expect("thread panicked");
    }
}


#[test]
fn independent_rng_instances() {
    let mut handles = Vec::new();

    for _ in 0..THREADS {
        handles.push(thread::spawn(|| {
            let mut rng = HadronRng::new()
                .expect("failed to create HadronRng");

            let p1 = rng.password(64)
                .expect("password");

            let p2 = rng.password(64)
                .expect("password");

            assert_ne!(p1, p2);
        }));
    }

    for handle in handles {
        handle.join()
            .expect("thread panicked");
    }
}


#[test]
fn no_thread_panics() {
    let mut handles = Vec::new();

    for _ in 0..THREADS {
        handles.push(thread::spawn(|| {
            let mut rng = HadronRng::new()
                .expect("failed to create HadronRng");

            for _ in 0..500 {
                let _ = rng.password(32)
                    .expect("password");

                let _ = rng.key(256)
                    .expect("key");

                let _ = rng.random_bytes(64)
                    .expect("bytes");
            }
        }));
    }

    for handle in handles {
        assert!(handle.join().is_ok());
    }
}