**Goal:
[klib](http://attractivechaos.github.io/klib/#Khash%3A%20generic%20hash%20table)
is a C macro-based fast data structure library including a hash map called
khash. This is heavily used in bioinformatic algorithms like the short-read
aligner [BWA-MEM](https://github.com/lh3/bwa/tree/mem). This project uses the
[benchmark](https://attractivechaos.wordpress.com/2008/10/07/another-look-at-my-old-benchmark/)
used by the klib author (Heng Li) to compare kash to the Rust
`std::collections::HashMap`.**

## Getting started

    git clone https://github.com/gaberudy/hash_test.git
    cd hash_test
    cargo build --release
    ./target/release/hash_test --help
    ./target/release/hash_test

## Benchmark

```
# items | type   | alg    | time    | mem
-------------------------------------------
5000000 | int    | rust   | 0.847   | 24.6M
5000000 | int    | khash  | 0.262   | 32.7M
5000000 | str    | rust   | 1.298   | 47.4M
5000000 | str    | khash  | 0.951   | 32.8M
```

## Thoughts and Future Direction

Rust is looking fast, but not matching Heng Li’s hand-crafted C code for this
specific benchmark. I’m thinking the klib/khash may have a couple advantages:

1. Hash function speed. Although I tried various hash functions, and I don’t
   believe they are the bottleneck, I didn’t yet implement the khash function in
   rust (or in C and link it in). The current benchmarks are run using
   `FnvHashMap` from the `fnv` crate, which uses the HashMap data structure and
   its ability to specify a different hash function (the default is
   cryptographically secure and fnv is also used heavily in rustc). I also
   tested every hash functin in the
   [fasthash](https://docs.rs/fasthash/0.3.2/fasthash/) create, and none did
   better than `fnv` for this bechmark.

2. Data types used for keys. Rust use u64 as the output of the hash functions
   and thus the data type for the HashMap keys. Khash is using C “int” (32-bit).
   This I believe accounts for most of the mem and potentially performance
   difference. It would be interesting to build as HashMap32 that truncates all
   8-byte keys and stores them as 4-byte u32 to compare to kash.

3. In particular khash does not hash int as key types, uses them directly as
   keys to the hash table! I think this explains the performance compared to
   hashing ints in rust, but note rust gets better space efficiency with the
   hashed integers than the less distributed in the key-space input integers.
