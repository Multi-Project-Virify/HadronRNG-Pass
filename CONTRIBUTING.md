Contributing to HadronRNG 

First of all, thank you for your interest in contributing to HadronRNG.

The goal of this project is to build a fast, secure and minimal cryptographic password and key generator written in Rust.

Every contribution is appreciated.

Ways to Contribute 

You can contribute by:

Reporting bugs Suggesting new features Improving documentation Writing tests Improving performance Reviewing pull requests Improving security Improving code quality Before You Start 

Please:

Read the documentation. Search existing issues before opening a new one. Keep pull requests focused on a single topic. Write clear commit messages. Add tests for new functionality whenever possible. Development Setup 

Clone the repository:

git clone https://github.com/YOUR_USERNAME/HadronRNG.git cd HadronRNG 

Build:

cargo build 

Run tests:

cargo test 

Run benchmarks:

cargo bench 

Format the code:

cargo fmt 

Run Clippy:

cargo clippy --all-targets --all-features 

Generate documentation:

cargo doc --open Coding Guidelines 

Please follow these principles:

Keep the code simple. Keep functions small and focused. Avoid unnecessary dependencies. Prefer readability over clever code. Write documentation for public APIs. Handle errors using Result. Avoid unwrap() and expect() in library code unless there is a compelling reason. Pull Requests 

A pull request should:

Build successfully. Pass all tests. Pass formatting checks. Pass Clippy checks. Include documentation for public APIs. Include tests for new functionality when appropriate. 

Large pull requests should be discussed before implementation.

Security Contributions 

Security improvements are always welcome.

If your contribution fixes a security vulnerability, please follow the instructions in SECURITY.md.

Do not disclose undisclosed vulnerabilities publicly before a fix is available.

Documentation 

Documentation improvements are welcome, including:

API documentation Examples Tutorials Architecture Performance notes Commit Messages 

Recommended format:

feat: add password policy builder fix: correct entropy validation docs: improve README examples perf: optimize random byte generation test: add stress tests refactor: simplify generator implementation Code Style 

The project uses:

rustfmt Clippy 

Please ensure your code passes both before submitting a pull request.

License 

By contributing to HadronRNG, you agree that your contributions will be licensed under the Apache License 2.0.

Thank you for helping improve HadronRNG.

