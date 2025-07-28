# Fungible Token Project 🪙

A complete Rust project for deploying and interacting with a NEAR Fungible Token (NEP-141) contract.

## 📁 Project Structure

```
ft-project/
├── contracts/
│   └── ft/                 # Fungible Token contract
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs      # NEP-141 implementation
├── scripts/                # Rust scripts for deployment and interaction
│   ├── create_account.rs   # Create new NEAR accounts
│   ├── deploy_contract.rs  # Deploy FT contract
│   ├── interact_with_ft.rs # Interact with deployed FT
│   └── check_token_state.rs # Check token state and info
├── tests/
│   └── ft_tests.rs         # Integration tests
├── src/
│   └── lib.rs              # Utility functions
├── Cargo.toml              # Workspace configuration
└── README.md
```

## 🚀 Getting Started

### Prerequisites

- Rust (with `wasm32-unknown-unknown` target)
- NEAR CLI (optional, for account creation)
- A NEAR testnet account with funds (for deployment)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install NEAR CLI (optional)
npm install -g near-cli-rs
```

### Setup

1. Clone this project
2. Create a `.env` file with your credentials:

```env
# For creating new accounts (optional)
FUNDER_ACCOUNT_ID=your-funded-account.testnet
FUNDER_SECRET_KEY=ed25519:your-secret-key

# For deployment (will be auto-generated if using create-account)
ACCOUNT_ID=your-account.testnet
PRIVATE_KEY=ed25519:your-private-key

# FT Configuration
FT_NAME=My Token
FT_SYMBOL=MTK
FT_DECIMALS=8
FT_TOTAL_SUPPLY=1000000000000000
```

## 📝 Scripts

### 1. Create Account
Creates a new NEAR testnet account:

```bash
cargo run --bin create-account
```

This will:
- Generate a new key pair
- Create a new testnet account (if you have a funder account)
- Save credentials to `.env`

### 2. Deploy Contract
Deploys the FT contract to your account:

```bash
cargo run --bin deploy
```

This will:
- Build the FT contract
- Deploy it to your account
- Initialize with specified parameters
- Save the contract ID to `.env`

### 3. Interact with Token
Interactive menu for token operations:

```bash
cargo run --bin interact
```

Options:
1. Check token balance
2. Transfer tokens
3. Check total supply
4. View token metadata
5. Register new accounts for storage

### 4. Check Token State
View comprehensive token information:

```bash
cargo run --bin check-token
```

Shows:
- Token metadata (name, symbol, decimals)
- Total supply
- Owner balance
- Storage requirements
- Contract information

## 🧪 Testing

Run integration tests:

```bash
cargo test
```

Tests include:
- Token initialization
- Transfers between accounts
- Metadata verification
- Storage management

## 🏗️ Building the Contract

To build the contract manually:

```bash
cd contracts/ft
cargo build --release --target wasm32-unknown-unknown
```

The compiled WASM will be at:
`contracts/ft/target/wasm32-unknown-unknown/release/fungible_token.wasm`

## 📚 Contract Methods

### View Methods (no gas required)
- `ft_metadata()` - Get token metadata
- `ft_total_supply()` - Get total token supply
- `ft_balance_of(account_id)` - Get account balance
- `storage_balance_of(account_id)` - Check storage deposit
- `storage_balance_bounds()` - Get storage requirements

### Change Methods (requires gas)
- `new(owner_id, total_supply, metadata)` - Initialize contract
- `ft_transfer(receiver_id, amount)` - Transfer tokens
- `ft_transfer_call(receiver_id, amount, memo, msg)` - Transfer with callback
- `storage_deposit(account_id)` - Register account for storage
- `storage_withdraw(amount)` - Withdraw unused storage deposit

## 🔍 Example Usage

After deployment, you can interact with your token:

```rust
// Check balance
let balance = contract.view("ft_balance_of")
    .args_json(json!({ "account_id": "alice.testnet" }))
    .await?;

// Transfer tokens
let result = account.call(contract_id, "ft_transfer")
    .args_json(json!({
        "receiver_id": "bob.testnet",
        "amount": "1000000"
    }))
    .deposit(1) // 1 yoctoNEAR for security
    .transact()
    .await?;
```

## 📖 Resources

- [NEP-141 Standard](https://nomicon.io/Standards/FungibleToken/Core)
- [NEAR SDK Rust](https://github.com/near/near-sdk-rs)
- [NEAR Workspaces](https://github.com/near/near-workspaces-rs)
- [NEAR Documentation](https://docs.near.org)

## 🤝 Contributing

Feel free to submit issues and enhancement requests!

## 📄 License

This project is licensed under the MIT License. 