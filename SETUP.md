# Quick Setup Guide 🚀

## Prerequisites

1. Install Rust and the WASM target:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown
```

2. Install cargo-near (optional, for advanced deployment):
```bash
cargo install cargo-near --locked
```

## Quick Start

1. **Build the contract:**
```bash
make build
# or
cargo build --release --target wasm32-unknown-unknown --manifest-path contracts/ft/Cargo.toml
```

2. **Run tests:**
```bash
make test
# or
cargo test
```

3. **Create account and deploy (sandbox):**
```bash
# Create account and save credentials
make create-account

# Deploy the contract
make deploy
```

4. **Interact with the token:**
```bash
make interact
```

5. **Check token state:**
```bash
make check-token
```

## Important Notes

⚠️ **Sandbox vs Testnet/Mainnet:**
- The scripts use NEAR sandbox (local) environment for easy testing
- For testnet/mainnet deployment:
  1. Create an account at https://testnet.mynearwallet.com/
  2. Use NEAR CLI: `near account create-account`
  3. Update `.env` with your testnet credentials
  4. Use `cargo near deploy` or NEAR CLI for deployment

## Environment Variables

The `.env` file should contain:
```env
# Account credentials
ACCOUNT_ID=your-account.testnet
PRIVATE_KEY=ed25519:your-private-key

# Token configuration
FT_NAME=My Token
FT_SYMBOL=MTK
FT_DECIMALS=8
FT_TOTAL_SUPPLY=1000000000000000

# Contract ID (set after deployment)
FT_CONTRACT_ID=ft-contract.testnet
```

## Troubleshooting

1. **"WASM file not found"**: Run `make build` first
2. **Compilation errors**: Ensure you have the correct Rust toolchain
3. **Network errors**: Scripts use sandbox by default, no internet required

## Next Steps

- Modify the token parameters in `.env`
- Deploy to testnet using NEAR CLI
- Integrate with a frontend application
- Add custom token functionality 