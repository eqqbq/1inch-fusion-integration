use anyhow::Result;
use near_api::{signer, AccountId, NetworkConfig, Signer, NearToken};
use near_crypto::SecretKey;
use serde_json::json;
use std::str::FromStr;

/// Deploys a Fungible Token contract directly to your account
/// 
/// This simplified script:
/// 1. Reads your smart contract account credentials from .env file
/// 2. Builds the FT contract
/// 3. Deploys and initializes the contract to your account
///
/// Required environment variables:
/// - SMART_CONTRACT_ACCOUNT_ID: The account where the contract will be deployed
/// - SC_PRIVATE_KEY: The private key for the smart contract account
#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenv::dotenv().ok();
    
    println!("🚀 Deploying Fungible Token Contract (Simple) to NEAR Testnet\n");
    
    // ===== 1. LOAD CONFIGURATION =====
    
    // Smart contract account credentials (from .env)
    let account_id = std::env::var("SMART_CONTRACT_ACCOUNT_ID")
        .expect("❌ SMART_CONTRACT_ACCOUNT_ID not found in .env");
    let private_key = std::env::var("SC_PRIVATE_KEY")
        .expect("❌ SC_PRIVATE_KEY not found in .env");
    
    // Token configuration (from .env with defaults)
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
    let account: AccountId = account_id.parse()?;
    let secret_key = SecretKey::from_str(&private_key)?;
    let signer = Signer::new(Signer::from_secret_key(secret_key))?;
    
    // Connect to testnet
    let network = NetworkConfig::testnet();
    
    // Display configuration
    println!("📋 Configuration:");
    println!("   Account: {}", account_id);
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
    
    // ===== 4. DEPLOY AND INITIALIZE CONTRACT =====
    
    // COMMENTED OUT DEPLOYMENT - NOW JUST CALLING FUNCTION ON EXISTING CONTRACT
    
    println!("📤 Deploying contract to your account...");
    
    // Prepare initialization arguments
    let init_args = json!({
        "owner_id": account.to_string(),
        "total_supply": ft_total_supply,
    });
    
    println!("Init args: {}", serde_json::to_string_pretty(&init_args)?);
    
    // Deploy contract with initialization
    let deploy_result = near_api::Contract::deploy(account.clone())
        .use_code(wasm_code)
        .with_init_call("new_default_meta", init_args).unwrap()
        .with_signer(signer)
        .send_to(&network)
        .await?;
    
    println!("✅ Contract deployed and initialized!");
    println!("   Transaction: https://testnet.nearblocks.io/txns/{:?}", 
        deploy_result.transaction_outcome.id);
    println!();
    
    
    // // ===== 4. CALL FUNCTION ON EXISTING CONTRACT =====
    
    // println!("📤 Calling initialization function on existing contract...");
    
    // // Prepare initialization arguments
    // let init_args = json!({
    //     "owner_id": account,
    //     "total_supply": ft_total_supply,
    // });
    
    // println!("Init args: {}", serde_json::to_string_pretty(&init_args)?);
    
    // // Call initialization function on already deployed contract
    // let contract = near_api::Contract(account.clone());
    // let call_result = contract
    //     .call_function("new_default_meta", init_args)?
    //     .transaction()
    //     .deposit(NearToken::from_yoctonear(0)) // No deposit needed for init
    //     .with_signer(account.clone(), signer.clone())
    //     .send_to(&network)
    //     .await?;
    
    // println!("✅ Function called successfully!");
    // println!("   Transaction: https://testnet.nearblocks.io/txns/{:?}", 
    //     call_result.transaction_outcome.id);
    // println!();
    
    // ===== 5. VERIFY DEPLOYMENT =====
    
    println!("🔍 Verifying deployment...");
    
    let contract = near_api::Contract(account.clone());
    
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
        "account_id": account.to_string()
    });
    
    let owner_balance: String = contract
        .call_function("ft_balance_of", balance_args)
        .unwrap()
        .read_only()
        .fetch_from(&network)
        .await?
        .data;
    
    println!("✓ Owner has full supply: {} raw units", owner_balance);
    
    // ===== 6. SUMMARY =====
    
    println!("\n🎉 Deployment Complete!");
    println!("\n📌 Token Contract: {}", account_id);
    println!("📌 View on Explorer: https://testnet.nearblocks.io/address/{}", account_id);
    println!("\n⚠️  Note: The contract is deployed to your main account, not a subaccount");
    println!("\n💡 Next steps:");
    println!("   - Run 'cargo run --bin interact' to transfer tokens");
    println!("   - Import token to NEAR Wallet using your account ID");
    
    Ok(())
} 