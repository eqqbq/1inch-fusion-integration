use anyhow::Result;
use near_api::{signer, Account, AccountId, NearToken, NetworkConfig, Signer};
use near_crypto::SecretKey;
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};
use std::str::FromStr;

/// Deploys a Fungible Token contract to NEAR testnet
/// 
/// This script:
/// 1. Reads configuration from .env file
/// 2. Creates a new subaccount from your parent account
/// 3. Builds the FT contract
/// 4. Deploys and initializes the contract
/// 5. Saves deployment info for future use
#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenv::dotenv().ok();
    
    println!("🚀 Deploying Fungible Token Contract to NEAR Testnet\n");
    
    // ===== 1. LOAD CONFIGURATION =====
    
    // Parent account credentials (from .env)
    let parent_account_id = std::env::var("PARENT_ACCOUNT_ID")
        .expect("❌ PARENT_ACCOUNT_ID not found in .env");
    let parent_private_key = std::env::var("PARENT_PRIVATE_KEY")
        .expect("❌ PARENT_PRIVATE_KEY not found in .env");
    
    // Token configuration (from .env with defaults)
    let subaccount_prefix = std::env::var("SUBACCOUNT_PREFIX")
        .unwrap_or("ft".to_string());
    let ft_name = std::env::var("FT_NAME")
        .unwrap_or("Example Token".to_string());
    let ft_symbol = std::env::var("FT_SYMBOL")
        .unwrap_or("EXT".to_string());
    let ft_decimals: u8 = std::env::var("FT_DECIMALS")
        .unwrap_or("8".to_string())
        .parse()?;
    let ft_total_supply = std::env::var("FT_TOTAL_SUPPLY")
        .unwrap_or("1000000000000000".to_string()); // 10M tokens with 8 decimals
    
    // ===== 2. SETUP NEAR CONNECTION =====
    
    // Parse account and create signer
    let parent_account: AccountId = parent_account_id.parse()?;
    let private_key = SecretKey::from_str(&parent_private_key)?;
    let signer = Signer::new(Signer::from_secret_key(private_key))?;
    
    // Connect to testnet
    let network = NetworkConfig::testnet();
    
    // Generate unique subaccount name with timestamp
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs();
    let subaccount_id: AccountId = format!("{}-{}.{}", subaccount_prefix, timestamp, parent_account_id)
        .parse()?;
    
    // Display configuration
    println!("📋 Configuration:");
    println!("   Parent Account: {}", parent_account_id);
    println!("   New Subaccount: {}", subaccount_id);
    println!("   Token Name: {}", ft_name);
    println!("   Token Symbol: {}", ft_symbol);
    println!("   Decimals: {}", ft_decimals);
    println!("   Total Supply: {} (raw units)", ft_total_supply);
    
    // Convert to human readable amount
    let human_readable = ft_total_supply.parse::<f64>()? / 10f64.powi(ft_decimals as i32);
    println!("   Total Supply: {} {} (human readable)", human_readable, ft_symbol);
    println!();
    
    // ===== 3. BUILD CONTRACT =====
    
    println!("📦 Building contract...");
    let build_output = std::process::Command::new("cargo")
        .args(&["near", "build", "non-reproducible-wasm"])
        .current_dir("contracts")
        .output()?;
    
    if !build_output.status.success() {
        anyhow::bail!("❌ Failed to build contract: {}", 
            String::from_utf8_lossy(&build_output.stderr));
    }
    
    // Read the compiled WASM file
    let wasm_path = "contracts/ft/target/near/fungible_token.wasm";
    let wasm_code = std::fs::read(wasm_path)?;
    
    println!("✅ Contract built successfully");
    println!("   WASM size: {} KB", wasm_code.len() / 1024);
    println!();
    
    // ===== 4. CREATE SUBACCOUNT =====
    
    println!("👶 Creating subaccount...");
    
    // Generate new keypair for the subaccount
    let new_private_key = signer::generate_secret_key()?;
    
    // Create the subaccount, funded by parent account
    let create_result = Account::create_account(subaccount_id.clone())
        .fund_myself(
            parent_account.clone(),
            NearToken::from_millinear(3000), // 3 NEAR initial balance
        )
        .public_key(new_private_key.public_key())?
        .with_signer(signer.clone())
        .send_to(&network)
        .await?;
    

    println!("✅ Subaccount created!");
    println!("   Transaction: https://testnet.nearblocks.io/txns/{:?}", 
        create_result.transaction_outcome.id);
    println!();
    
    // ===== 5. DEPLOY AND INITIALIZE CONTRACT =====
    
    println!("📤 Deploying contract...");
    
    // Prepare initialization arguments
    let init_args = json!({
        "owner_id": subaccount_id.to_string(),
        "total_supply": ft_total_supply,
        "metadata": {
            "spec": "ft-1.0.0",
            "name": ft_name,
            "symbol": ft_symbol,
            "decimals": ft_decimals,
        }
    });
    
    println!("{:?}", init_args);
    
    // Create signer for the new subaccount
    let subaccount_signer = Signer::new(Signer::from_secret_key(new_private_key.clone()))?;
    
    // Deploy contract with initialization
    let deploy_result = near_api::Contract::deploy(subaccount_id.clone())
        .use_code(wasm_code)
        .with_init_call("new", init_args)?
        .with_signer(subaccount_signer)
        .send_to(&network)
        .await?;
    

    println!("{:?}", deploy_result);

    println!("✅ Contract deployed and initialized!");
    println!("   Transaction: https://testnet.nearblocks.io/txns/{:?}", 
        deploy_result.transaction_outcome.id);
    println!();
    
    // ===== 6. VERIFY DEPLOYMENT =====
    
    println!("🔍 Verifying deployment...");
    
    let contract = near_api::Contract(subaccount_id.clone());
    
    // Get token metadata
    let metadata: serde_json::Value = contract
        .call_function("ft_metadata", ())
        .unwrap()
        .read_only()
        .fetch_from(&network)
        .await?
        .data;
    
    println!("✓ Token metadata verified:");
    println!("  {}", serde_json::to_string_pretty(&metadata)?);
    
    // Check owner balance
    let balance_args = json!({
        "account_id": subaccount_id.to_string()
    });
    
    let owner_balance: String = contract
        .call_function("ft_balance_of", balance_args)
        .unwrap()
        .read_only()
        .fetch_from(&network)
        .await?
        .data;
    
    println!("✓ Owner has full supply: {} raw units", owner_balance);
    
    // ===== 7. SAVE DEPLOYMENT INFO =====
    
    println!("\n💾 Saving deployment info...");
    
    let deployment_info = format!(
        "# FT Contract Deployment Info\n\
        # Generated at: {}\n\
        \n\
        # Contract Account\n\
        FT_CONTRACT_ID={}\n\
        FT_CONTRACT_PRIVATE_KEY={}\n\
        \n\
        # Parent Account\n\
        PARENT_ACCOUNT={}\n\
        \n\
        # Token Details\n\
        FT_NAME={}\n\
        FT_SYMBOL={}\n\
        FT_DECIMALS={}\n\
        FT_TOTAL_SUPPLY={}\n",
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
        subaccount_id,
        new_private_key.to_string(),
        parent_account_id,
        ft_name,
        ft_symbol,
        ft_decimals,
        ft_total_supply
    );
    
    std::fs::write("deployment-info.env", deployment_info)?;
    println!("✅ Deployment info saved to deployment-info.env");
    
    // ===== 8. SUMMARY =====
    
    println!("\n🎉 Deployment Complete!");
    println!("\n📌 Token Contract: {}", subaccount_id);
    println!("📌 View on Explorer: https://testnet.nearblocks.io/address/{}", subaccount_id);
    println!("\n💡 Next steps:");
    println!("   - Run 'cargo run --bin interact' to transfer tokens");
    println!("   - Check deployment-info.env for contract details");
    println!("   - Import token to NEAR Wallet using contract ID");
    
    Ok(())
} 