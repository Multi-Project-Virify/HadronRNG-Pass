# HadronRNG-Pass Architecture


## Overview

HadronRNG-Pass is a modular random generation library
written in Rust.

The architecture is designed around separation of
responsibilities:

- Entropy collection
- Generator core
- State management
- Password generation
- Key generation
- Validation
- Memory handling


# Architecture Structure


```text
HadronRNG-Pass

src/
├── lib.rs
├── error.rs
├── config.rs
├── charset.rs
├── policy.rs
├── entropy.rs
├── state.rs
├── generator.rs
├── password.rs
├── key.rs
├── validator.rs
├── memory.rs
└── utils.rs