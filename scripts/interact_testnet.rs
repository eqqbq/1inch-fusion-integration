use anyhow::Result;
use near_workspaces::types::{NearToken, SecretKey};
use near_workspaces::network::Testnet;
use near_workspaces::Worker;
use serde_json::json;

// Constants for the transfer
const RECIPIENT_ACCOUNT: &str = "holoo.testnet"; // Change to your recipient
const TRANSFER_AMOUNT: &str = "1000000000"; // 10 tokens with 8 decimals

#[tokio::main]
async fn main() -> Result<()> {
    // Load from .env file
    dotenv::dotenv().ok();
    
    println!("🔄 Interacting with FT Contract on Testnet...\n");
    
    // Get configuration from environment
    let account_id = std::env::var("PARENT_ACCOUNT_ID")
        .expect("PARENT_ACCOUNT_ID not found in .env");
    let private_key = std::env::var("PARENT_PRIVATE_KEY")
        .expect("PARENT_PRIVATE_KEY not found in .env");
    let subaccount_prefix = std::env::var("SUBACCOUNT_PREFIX")
        .unwrap_or("ft".to_string());
    
    // Construct the contract ID (subaccount)
    let ft_contract_id = format!("{}.{}", subaccount_prefix, account_id);
    
    println!("📋 Configuration:");
    println!("   Your Account: {}", account_id);
    println!("   FT Contract: {}", ft_contract_id);
    println!("   Recipient: {}", RECIPIENT_ACCOUNT);
    println!("   Transfer Amount: {} (base units)", TRANSFER_AMOUNT);
    println!();
    
    // Initialize testnet worker
    let worker = Worker::<Testnet>::testnet().await?;
    
    // Import your account
    println!("🔑 Importing account...");
    let sk: SecretKey = private_key.parse()?;
    let account = worker.import_account(&account_id.parse()?, &sk).await?;
    println!("✅ Account imported\n");
    
    // 1. Check token metadata
    println!("📋 Token Information:");
    let metadata: serde_json::Value = account
        .view(&ft_contract_id.parse()?, "ft_metadata")
        .await?
        .json()?;
    
    println!("   Name: {}", metadata["name"]);
    println!("   Symbol: {}", metadata["symbol"]);
    println!("   Decimals: {}", metadata["decimals"]);
    
    // 2. Check your balance
    println!("\n💰 Checking your balance...");
    let balance: String = account
        .view(&ft_contract_id.parse()?, "ft_balance_of")
        .args_json(json!({
            "account_id": account_id
        }))
        .await?
        .json()?;
    
    let decimals = metadata["decimals"].as_u64().unwrap_or(0);
    let balance_float = balance.parse::<f64>().unwrap_or(0.0) / 10f64.powi(decimals as i32);
    
    println!("   Your Balance: {} {} ({} base units)", 
        balance_float, 
        metadata["symbol"], 
        balance
    );
    
    // 3. Check if recipient has storage (if not, register them)
    println!("\n🔍 Checking recipient storage...");
    let recipient_storage: serde_json::Value = account
        .view(&ft_contract_id.parse()?, "storage_balance_of")
        .args_json(json!({
            "account_id": RECIPIENT_ACCOUNT
        }))
        .await?
        .json()?;
    
    if recipient_storage.is_null() {
        println!("   ⚠️  Recipient not registered. Registering...");
        
        let register_result = account
            .call(&ft_contract_id.parse()?, "storage_deposit")
            .args_json(json!({
                "account_id": RECIPIENT_ACCOUNT
            }))
            .deposit(NearToken::from_millinear(5)) // 0.005 NEAR for storage
            .transact()
            .await?;
        
        if register_result.is_success() {
            println!("   ✅ Recipient registered for token storage");
        } else {
            println!("   ❌ Failed to register recipient");
            return Ok(());
        }
    } else {
        println!("   ✅ Recipient already registered");
    }
    
    // 4. Transfer tokens
    println!("\n💸 Transferring tokens...");
    println!("   From: {}", account_id);
    println!("   To: {}", RECIPIENT_ACCOUNT);
    println!("   Amount: {} base units", TRANSFER_AMOUNT);
    
    let transfer_result = account
        .call(&ft_contract_id.parse()?, "ft_transfer")
        .args_json(json!({
            "receiver_id": RECIPIENT_ACCOUNT,
            "amount": TRANSFER_AMOUNT
        }))
        .deposit(NearToken::from_yoctonear(1)) // 1 yoctoNEAR for security
        .transact()
        .await?;
    
    if transfer_result.is_success() {
        println!("   ✅ Transfer successful!");
        
        // Check new balances
        println!("\n📊 Updated Balances:");
        
        // Your new balance
        let new_balance: String = account
            .view(&ft_contract_id.parse()?, "ft_balance_of")
            .args_json(json!({
                "account_id": account_id
            }))
            .await?
            .json()?;
        
        let new_balance_float = new_balance.parse::<f64>().unwrap_or(0.0) / 10f64.powi(decimals as i32);
        println!("   Your Balance: {} {} ({} base units)", 
            new_balance_float, 
            metadata["symbol"], 
            new_balance
        );
        
        // Recipient balance
        let recipient_balance: String = account
            .view(&ft_contract_id.parse()?, "ft_balance_of")
            .args_json(json!({
                "account_id": RECIPIENT_ACCOUNT
            }))
            .await?
            .json()?;
        
        let recipient_balance_float = recipient_balance.parse::<f64>().unwrap_or(0.0) / 10f64.powi(decimals as i32);
        println!("   Recipient Balance: {} {} ({} base units)", 
            recipient_balance_float, 
            metadata["symbol"], 
            recipient_balance
        );
        
        // Transaction info
        println!("\n🔗 Transaction completed on testnet");
        println!("   View on explorer: https://testnet.nearblocks.io/address/{}", ft_contract_id);
        
    } else {
        println!("   ❌ Transfer failed!");
        println!("   Error: {:?}", transfer_result);
    }
    
    Ok(())
} 