use anyhow::Result;
use near_api::{AccountId, Contract, NearToken, NetworkConfig, Signer};
use near_crypto::SecretKey;
use serde_json::json;
use std::str::FromStr;

// ===== CONFIGURATION =====
// Change these values to customize the transfer
const RECIPIENT_ACCOUNT: &str = "holoo.testnet";  // Who receives tokens
const TRANSFER_AMOUNT: &str = "1000000000";       // 10 tokens (with 8 decimals)

/// Interacts with a deployed Fungible Token contract
/// 
/// This script:
/// 1. Connects to your FT contract on testnet
/// 2. Shows token metadata and your balance
/// 3. Transfers tokens to a recipient
/// 4. Shows updated balances
#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenv::dotenv().ok();
    
    println!("🔄 Interacting with Fungible Token Contract\n");
    
    // ===== 1. LOAD CONFIGURATION =====
    
    // Your account credentials (from .env)
    let account_id = std::env::var("PARENT_ACCOUNT_ID")
        .expect("❌ PARENT_ACCOUNT_ID not found in .env");
    let private_key_string = std::env::var("PARENT_PRIVATE_KEY")
        .expect("❌ PARENT_PRIVATE_KEY not found in .env");
    
    // Contract location (from .env with default)
    let subaccount_prefix = std::env::var("SUBACCOUNT_PREFIX")
        .unwrap_or("ft".to_string());
    
    // ===== 2. SETUP NEAR CONNECTION =====
    
    // Parse account and create signer
    let account: AccountId = account_id.parse()?;
    let private_key = SecretKey::from_str(&private_key_string)?;
    let signer = Signer::new(Signer::from_secret_key(private_key))?;
    
    // Connect to testnet
    let network = NetworkConfig::testnet();
    
    // Build contract ID (subaccount format)
    let ft_contract_id: AccountId = format!("{}.{}", subaccount_prefix, account_id).parse()?;
    
    // Display configuration
    println!("📋 Configuration:");
    println!("   Your Account: {}", account_id);
    println!("   FT Contract: {}", ft_contract_id);
    println!("   Recipient: {}", RECIPIENT_ACCOUNT);
    println!("   Transfer Amount: {} (raw units)", TRANSFER_AMOUNT);
    println!();
    
    // Create contract object for interactions
    let contract = Contract(ft_contract_id.clone());
    
    // ===== 3. GET TOKEN INFORMATION =====
    
    println!("📊 Getting token information...\n");
    
    // Fetch token metadata
    let metadata: serde_json::Value = contract
        .call_function("ft_metadata", ())
        .unwrap()
        .read_only()
        .fetch_from(&network)
        .await?
        .data;
    
    // Extract important fields
    let token_name = metadata["name"].as_str().unwrap_or("Unknown");
    let token_symbol = metadata["symbol"].as_str().unwrap_or("???");
    let decimals = metadata["decimals"].as_u64().unwrap_or(0);
    
    println!("Token: {} ({})", token_name, token_symbol);
    println!("Decimals: {}", decimals);
    println!("Spec: {}", metadata["spec"].as_str().unwrap_or(""));
    println!();
    
    // ===== 4. CHECK YOUR BALANCE =====
    
    println!("💰 Checking your balance...");
    
    let balance_args = json!({
        "account_id": account_id
    });
    
    // Get your balance
    let your_balance: String = contract
        .call_function("ft_balance_of", balance_args.clone())
        .unwrap()
        .read_only()
        .fetch_from(&network)
        .await?
        .data;
    
    // Convert to human readable format
    let balance_float = your_balance.parse::<f64>().unwrap_or(0.0) / 10f64.powi(decimals as i32);
    
    println!("Your balance: {} {} ({} raw units)", 
        balance_float, token_symbol, your_balance);
    
    // Check if you have enough balance
    let transfer_amount_u128: u128 = TRANSFER_AMOUNT.parse()?;
    let balance_u128: u128 = your_balance.parse()?;
    
    if balance_u128 < transfer_amount_u128 {
        anyhow::bail!("❌ Insufficient balance! You have {} but trying to transfer {}", 
            your_balance, TRANSFER_AMOUNT);
    }
    println!();
    
    // ===== 5. CHECK RECIPIENT STORAGE =====
    
    println!("🔍 Checking recipient account...");
    
    let recipient_args = json!({
        "account_id": RECIPIENT_ACCOUNT
    });
    
    // Try to get recipient balance
    let recipient_balance_result: Result<near_api::Data<String>, _> = contract
        .call_function("ft_balance_of", recipient_args.clone())
        .unwrap()
        .read_only()
        .fetch_from(&network)
        .await;
    
    // Check if recipient needs storage registration
    let needs_storage = recipient_balance_result.is_err() || 
        recipient_balance_result.as_ref().unwrap().data == "0";
    
    if needs_storage {
        println!("⚠️  Recipient needs storage registration");
        println!("📝 Registering storage for {}...", RECIPIENT_ACCOUNT);
        
        let register_args = json!({
            "account_id": RECIPIENT_ACCOUNT
        });
        
        // Register storage for recipient
        let register_result = contract
            .call_function("storage_deposit", register_args)
            .unwrap()
            .transaction()
            .deposit(NearToken::from_millinear(2u128)) // 0.002 NEAR for storage
            .with_signer(account.clone(), signer.clone())
            .send_to(&network)
            .await?;
        
        println!("✅ Storage registered successfully");
        println!("   Transaction: https://testnet.nearblocks.io/txns/{:?}", 
            register_result.transaction_outcome.id);
    } else {
        println!("✅ Recipient already has storage");
    }
    println!();
    
    // ===== 6. TRANSFER TOKENS =====
    
    let transfer_human = transfer_amount_u128 as f64 / 10f64.powi(decimals as i32);
    println!("📤 Transferring {} {} to {}...", 
        transfer_human, token_symbol, RECIPIENT_ACCOUNT);
    
    let transfer_args = json!({
        "receiver_id": RECIPIENT_ACCOUNT,
        "amount": TRANSFER_AMOUNT,
        "memo": "Transfer from Rust script"
    });
    
    // Execute the transfer
    let transfer_result = contract
        .call_function("ft_transfer", transfer_args)
        .unwrap()
        .transaction()
        .deposit(NearToken::from_yoctonear(1)) // 1 yoctoNEAR required
        .with_signer(account.clone(), signer)
        .send_to(&network)
        .await?;
    
    println!("✅ Transfer successful!");
    println!("   Transaction: https://testnet.nearblocks.io/txns/{:?}", 
        transfer_result.transaction_outcome.id);
    println!();
    
    // ===== 7. CHECK UPDATED BALANCES =====
    
    println!("📊 Updated balances:");
    
    // Your new balance
    let your_final_balance: String = contract
        .call_function("ft_balance_of", balance_args)
        .unwrap()
        .read_only()
        .fetch_from(&network)
        .await?
        .data;
    
    let your_final_float = your_final_balance.parse::<f64>().unwrap_or(0.0) / 10f64.powi(decimals as i32);
    
    // Recipient's new balance
    let recipient_final_balance: String = contract
        .call_function("ft_balance_of", recipient_args)
        .unwrap()
        .read_only()
        .fetch_from(&network)
        .await?
        .data;
    
    let recipient_final_float = recipient_final_balance.parse::<f64>().unwrap_or(0.0) / 10f64.powi(decimals as i32);
    
    // Display balances in a nice table format
    println!("   ┌─────────────────────┬──────────────┬──────────────┐");
    println!("   │ Account             │ Balance      │ Raw Units    │");
    println!("   ├─────────────────────┼──────────────┼──────────────┤");
    println!("   │ You                 │ {:>10.2} {} │ {:>12} │", 
        your_final_float, token_symbol, your_final_balance);
    println!("   │ Recipient           │ {:>10.2} {} │ {:>12} │", 
        recipient_final_float, token_symbol, recipient_final_balance);
    println!("   └─────────────────────┴──────────────┴──────────────┘");
    
    println!("\n✅ All done!");
    println!("\n💡 Tips:");
    println!("   - View your transactions at: https://testnet.nearblocks.io/address/{}", account_id);
    println!("   - Change RECIPIENT_ACCOUNT in the script to send to someone else");
    println!("   - Modify TRANSFER_AMOUNT to send a different amount");
    
    Ok(())
} 