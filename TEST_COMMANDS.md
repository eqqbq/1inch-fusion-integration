# FT-Project Testing Commands

## Prerequisites
Make sure you have the following installed:
- Rust (latest stable)
- NEAR CLI (optional, for deployment testing)
- wasm32-unknown-unknown target: `rustup target add wasm32-unknown-unknown`

## Contract Testing Commands

### From the contracts directory (ft-project/contracts/):

```bash

#
cargo near build non-reproducible-wasm
# Run all tests (unit + integration)
cargo test

# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test ft_tests

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_ft_transfer

# Run tests in release mode (faster)
cargo test --release

# Check code without building
cargo check

# Format code
cargo fmt

# Run clippy (linter)
cargo clippy -- -D warnings

# Build and optimize the WASM file
cargo build --release --target wasm32-unknown-unknown
# The WASM file will be at: target/wasm32-unknown-unknown/release/fungible_token.wasm
```


## Script Testing Commands

### Deployment Script Testing

```bash
# From ft-project directory
# First, set up your environment variables
cp .env.example .env  # If you have an example
# Edit .env to add your account credentials

# Run the deploy script
cargo run --bin deploy

# Deploy with specific parameters
NEAR_ACCOUNT_ID=your-account.testnet cargo run --bin deploy
```

### Interaction Script Testing

```bash
# From ft-project directory
# Run the interact script
cargo run --bin interact

```

## Integration Testing with NEAR Sandbox

The integration tests use near-workspaces which automatically starts a local sandbox:

```bash
# From contracts directory
# Run integration tests with detailed output
RUST_LOG=near_workspaces=info cargo test --test ft_tests -- --nocapture

# Run a specific integration test
cargo test test_ft_transfer -- --nocapture

# Run with backtrace for debugging
RUST_BACKTRACE=1 cargo test
```

## Complete Test Suite Command

Run this from the ft-project directory to test everything:

```bash
#!/bin/bash
echo "=== Running FT Project Complete Test Suite ==="

echo "1. Checking code format..."
cd contracts && cargo fmt --check

echo "2. Running clippy..."
cargo clippy -- -D warnings

echo "3. Building contract..."
cargo build --release --target wasm32-unknown-unknown

echo "4. Running unit tests..."
cargo test --lib

echo "5. Running integration tests..."
cargo test --test ft_tests

echo "6. Contract built at:"
ls -la target/wasm32-unknown-unknown/release/*.wasm

cd ..
echo "7. Building scripts..."
cargo build --bin deploy --bin interact

echo "=== All tests completed! ==="
```

## Quick Test Commands

```bash
# Quick test everything (from ft-project/contracts)
cargo test

# Quick check everything compiles (from ft-project)
cargo check --all

# Test with all features
cargo test --all-features
```

## Debugging Failed Tests

```bash
# Run with backtrace
RUST_BACKTRACE=1 cargo test

# Run with full backtrace
RUST_BACKTRACE=full cargo test

# Run single test with print output
cargo test test_name -- --nocapture --test-threads=1

# Check test compilation only
cargo test --no-run
```

## Performance Testing

```bash
# Run benchmarks (if you add any)
cargo bench

# Test in release mode for performance
cargo test --release

# Profile test execution time
cargo test -- --nocapture --test-threads=1
```

## Coverage Testing (Optional - requires cargo-tarpaulin)

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Run coverage from contracts directory
cargo tarpaulin --out Html --output-dir coverage
```

## Clean and Rebuild

```bash
# Clean all build artifacts
cargo clean

# Clean and rebuild everything
cargo clean && cargo test
```

## Common Workflow

Here's a typical testing workflow:

```bash
# 1. Go to contracts directory
cd ft-project/contracts

# 2. Format and lint
cargo fmt && cargo clippy -- -D warnings

# 3. Run all tests
cargo test

# 4. Build the contract
cargo build --release --target wasm32-unknown-unknown

# 5. Go back to main project for scripts
cd ..

# 6. Test deployment script (dry run)
cargo check --bin deploy

# 7. Test interaction script (dry run)
cargo check --bin interact
``` 