# Deploy to Testnet Instructions 🚀

## Prerequisites

1. Make sure you have the WASM target installed:
```bash
rustup target add wasm32-unknown-unknown
```

2. Your `.env` file is configured with your account credentials

## Deployment Steps

### 1. Review Configuration

Check your `.env` file contains:
- `PARENT_ACCOUNT_ID` - Your testnet account (e.g., holootest.testnet)
- `PARENT_PRIVATE_KEY` - Your private key (ed25519:...)
- `SUBACCOUNT_PREFIX` - Prefix for the subaccount (default: ft)
- Token parameters (name, symbol, decimals, supply)

### 2. Build the Contract

```bash
cargo build --release --target wasm32-unknown-unknown --manifest-path contracts/ft/Cargo.toml
```

### 3. Deploy to Testnet

```bash
cargo run --bin deploy-from-keystore
```

This will:
1. Import your parent account (holootest.testnet)
2. Create a subaccount (ft.holootest.testnet)
3. Deploy the FT contract to the subaccount
4. Initialize the token with your specified parameters
5. Save deployment info to `deployment-info.env`

## What Happens During Deployment

1. **Account Import**: Uses your private key to access the parent account
2. **Subaccount Creation**: Creates `{SUBACCOUNT_PREFIX}.{PARENT_ACCOUNT_ID}`
3. **Contract Deployment**: Uploads the WASM to the subaccount
4. **Initialization**: Calls the `new` method with token metadata
5. **Verification**: Checks that the token was deployed correctly

## After Deployment

Your token will be live at: `ft.holootest.testnet` (or whatever subaccount you configured)

You can:
- View it on: https://testnet.nearblocks.io/address/ft.holootest.testnet
- Interact with it using NEAR CLI or the interact script
- Transfer tokens to other accounts (they need to register first)

## Security Notes

⚠️ **Important**:
- Never commit `.env` to version control
- Keep your private key secure
- For production, use a hardware wallet or secure key management

## Troubleshooting

1. **"Account already exists"**: The subaccount was already created. Change the `SUBACCOUNT_PREFIX`
2. **"Not enough balance"**: Your parent account needs at least 5-10 NEAR for deployment
3. **"Contract too large"**: The compiled WASM exceeds size limits. Optimize your code 