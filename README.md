HadronRNG 

Fast, secure and minimal cryptographic password & key generator written in Rust.

Overview 

HadronRNG is an open-source Rust library focused on generating cryptographically secure passwords, random bytes and cryptographic keys.

The project is designed with the following goals:

Security first Minimal dependencies High performance Memory safety Clean and simple API Cross-platform support Features Cryptographically secure random password generation Cryptographic key generation Random byte generation Custom password policies Custom character sets Memory wiping for sensitive data Secure defaults Thread-safe design Cross-platform Planned Features Passphrase generator Diceware support Password strength analyzer Entropy estimation Additional output formats no_std support Optional CLI application Installation 

Add the crate to your project:

[dependencies] hadronrng = "0.1" Example use hadronrng::HadronRng; fn main() -> Result<(), Box<dyn std::error::Error>> { let mut rng = HadronRng::new()?; let password = rng.password(32)?; println!("{password}"); Ok(()) } Project Structure src/ ├── entropy.rs ├── generator.rs ├── password.rs ├── key.rs ├── charset.rs ├── policy.rs ├── validator.rs ├── memory.rs ├── statistics.rs └── lib.rs Security 

HadronRNG follows several design principles:

Cryptographically secure randomness Secure memory handling Defensive programming Continuous testing Fuzz testing Benchmarking 

Security issues should be reported privately. Please see SECURITY.md.

Benchmarks 

Performance benchmarks are located in the benches/ directory.

Run:

cargo bench Testing 

Run all tests:

cargo test 

Run fuzz tests:

cargo fuzz run Documentation 

API documentation:

cargo doc --open Contributing 

Contributions, bug reports and feature requests are welcome.

Please read CONTRIBUTING.md before submitting pull requests.

License 

Licensed under the Apache License, Version 2.0.

See LICENSE for details.

Acknowledgements 

Thanks to the Rust community and the maintainers of the open-source ecosystem for providing reliable tooling and libraries that make secure software development possible.

