# HadronRNG-Pass Testing Documentation


## Overview

This document describes the testing strategy used
for HadronRNG-Pass.

The project uses multiple testing layers to verify:

- Correctness
- Stability
- Performance
- Randomness quality
- Thread safety


# Testing Structure


Tests are located in:

```text
tests/
├── common/
│   └── mod.rs
├── api.rs
├── entropy.rs
├── generator.rs
├── password.rs
├── key.rs
├── charset.rs
├── policy.rs
├── validator.rs
├── memory.rs
├── stress.rs
├── regression.rs
├── parallel.rs
├── randomness.rs
└── integration.rs