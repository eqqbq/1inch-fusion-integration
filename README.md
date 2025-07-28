# Fungible Token Project 🪙

A complete Rust project for deploying and interacting with a NEAR Fungible Token (NEP-141) contract on testnet.

## 📁 Project Structure

```
ft-project/
├── contracts/
│   └── ft/                 # Fungible Token contract
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs      # NEP-141 implementation
├── scripts/                # Rust scripts for deployment and interaction
│   ├── deploy_from_keystore.rs  # Deploy to testnet using existing account
│   └── interact_testnet.rs      # Interact with deployed FT on testnet
├── tests/
│   └── ft_tests.rs         # Integration tests
├── src/
│   └── lib.rs              # Utility functions
├── Cargo.toml              # Workspace configuration
├── .env                    # Environment configuration (create from .env.example)
└── README.md
```

## 🚀 Getting Started

### Prerequisites

- Rust (with `wasm32-unknown-unknown` target)
- A NEAR testnet account with funds
- Your account's private key

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown
```

### Setup

1. Clone this project

2. **Create a testnet account** (if you don't have one):

   **Option A: Using NEAR CLI (Recommended)**
   ```bash
   # Install NEAR CLI
   npm install -g near-cli-rs

   # Create account with faucet funding
   near account create-account sponsor-by-faucet-service <your-name>.testnet autogenerate-new-keypair save-to-keychain network-config testnet create
   ```
   **Getting your credentials for .env:**
    
    Use Near CLI to check your account
3. Create a `.env` file with your testnet credentials:

```env
# Parent account credentials (testnet)
PARENT_ACCOUNT_ID=your-account.testnet
PARENT_PRIVATE_KEY=ed25519:your-private-key-here

# Subaccount configuration
# This will create: ft.your-account.testnet
SUBACCOUNT_PREFIX=ft

# Token configuration
FT_NAME=My Token
FT_SYMBOL=MTK
FT_DECIMALS=8
FT_TOTAL_SUPPLY=1000000000000000
```

## 📝 Scripts

### Deploy to Testnet
Deploy your fungible token contract:

```bash
cargo run --bin deploy
```

This script will:
- ✅ Build your contract automatically
- ✅ Create a new subaccount from your parent account
- ✅ Deploy and initialize the FT contract
- ✅ Save deployment info to `deployment-info.env`
- ✅ Show you the contract address and explorer link

The script displays progress at each step and provides helpful error messages if anything goes wrong.

### Interact with Your Token
Transfer tokens and check balances:

```bash
cargo run --bin interact
```

This script will:
- 📊 Show token metadata (name, symbol, decimals)
- 💰 Display your current balance
- 🔄 Transfer tokens to a recipient (configured in the script)
- 📈 Show updated balances in a nice table format

To customize the transfer:
- Edit `RECIPIENT_ACCOUNT` in `scripts/interact.rs` (default: holoo.testnet)
- Edit `TRANSFER_AMOUNT` in `scripts/interact.rs` (default: 10 tokens)

This script:
- Connects to your deployed FT contract
- Shows token metadata
- Checks your balance
- Transfers tokens to a recipient (configured in the script)
- Shows updated balances

To customize the transfer:
- Edit `scripts/interact_testnet.rs`
- Change `RECIPIENT_ACCOUNT` constant
- Change `TRANSFER_AMOUNT` constant (in base units)

## 🧪 Testing

Run integration tests (uses sandbox environment):

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

After deployment, your token will be live at: `ft.your-account.testnet`

### Check on Explorer
View your deployed token: `https://testnet.nearblocks.io/address/ft.your-account.testnet`

### Using the Scripts

1. **Deploy**:
   ```bash
   cargo run --bin deploy-from-keystore
   ```

2. **Interact**:
   ```bash
   cargo run --bin interact-testnet
   ```

### Manual Interaction with NEAR CLI

```bash
# Check balance
near view ft.your-account.testnet ft_balance_of '{"account_id": "your-account.testnet"}'

# Transfer tokens
near call ft.your-account.testnet ft_transfer '{"receiver_id": "recipient.testnet", "amount": "1000000000"}' --accountId your-account.testnet --depositYocto 1
```

## 🔑 Account Management


### Creating Additional Accounts

You can create more testnet accounts anytime:
```bash
# Create with custom name
near account create-account sponsor-by-faucet-service another-account.testnet autogenerate-new-keypair save-to-keychain network-config testnet create

# Create sub-account (requires existing account)
near account create-account fund-myself my-token.your-account.testnet 5 autogenerate-new-keypair save-to-keychain sign-as your-account.testnet network-config testnet create
```

## 🔐 Security Notes

⚠️ **Important**:
- **Never commit `.env` to version control** (it's in .gitignore)
- Keep your private key secure
- Never share your private key publicly
- For production, use a hardware wallet or secure key management
- The example uses testnet - be extra careful with mainnet

## 📖 Resources

- [NEP-141 Standard](https://nomicon.io/Standards/FungibleToken/Core)
- [NEAR SDK Rust](https://github.com/near/near-sdk-rs)
- [NEAR Workspaces](https://github.com/near/near-workspaces-rs)
- [NEAR Documentation](https://docs.near.org)
- [NEAR Testnet Explorer](https://testnet.nearblocks.io)

## 🤝 Contributing

Feel free to submit issues and enhancement requests!

## 📄 License

This project is licensed under the MIT License. 