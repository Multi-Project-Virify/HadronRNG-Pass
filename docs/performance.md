# HadronRNG-Pass Performance Documentation


## Overview

This document describes performance considerations,
benchmarking methods and optimization goals of
HadronRNG-Pass.


# Performance Goals

Main goals:

- Fast random data generation
- Low memory overhead
- Efficient password generation
- Scalable multi-thread operation


# Benchmark System


Performance tests are located in:

```text
benches/
├── rng_bench.rs
├── password_bench.rs
└── key_bench.rs