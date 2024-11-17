# Restaking CLI

## Getting started

## Run

### Code generation
```bash
cargo b -p jito-shank-cli && ./target/debug/jito-shank-cli && yarn generate-clients && cargo b
```

### Clippy
```bash
cargo +nightly-2024-07-25 clippy --all-features -- -D warnings -D clippy::all -D clippy::nursery -D clippy::integer_division -D clippy::arithmetic_side_effects -D clippy::style -D clippy::perf
```

### Format
```bash
cargo +nightly-2024-07-25 fmt --all
```

### Test
```bash
cargo-build-sbf && SBF_OUT_DIR=$(pwd)/target/sbf-solana-solana/release cargo nextest run --all-features
```

```bash
cargo nextest run --all-features
```

## Resources
- [Kinobi](https://github.com/kinobi-so/kinobi)
