use anyhow::Result;
use near_workspaces::types::{NearToken, SecretKey};
use near_workspaces::network::Testnet;
use near_workspaces::Worker;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    // Load from .env file
    dotenv::dotenv().ok();
    
    println!("🚀 Deploying FT Contract from Testnet Account...\n");
    
    // Parent account credentials from environment
    let parent_account_id = std::env::var("PARENT_ACCOUNT_ID")
        .expect("PARENT_ACCOUNT_ID not found in .env");
    let parent_private_key = std::env::var("PARENT_PRIVATE_KEY")
        .expect("PARENT_PRIVATE_KEY not found in .env");
    
    // FT configuration
    let ft_name = std::env::var("FT_NAME").unwrap_or("Example Token".to_string());
    let ft_symbol = std::env::var("FT_SYMBOL").unwrap_or("EXT".to_string());
    let ft_decimals = std::env::var("FT_DECIMALS")
        .unwrap_or("8".to_string())
        .parse::<u8>()?;
    let ft_total_supply = std::env::var("FT_TOTAL_SUPPLY")
        .unwrap_or("1000000000000000".to_string());
    
    // Subaccount name
    let subaccount_prefix = std::env::var("SUBACCOUNT_PREFIX").unwrap_or("ft".to_string());
    let subaccount_id = format!("{}.{}", subaccount_prefix, parent_account_id);
    
    println!("📋 Configuration:");
    println!("   Parent Account: {}", parent_account_id);
    println!("   Subaccount: {}", subaccount_id);
    println!("   Token Name: {}", ft_name);
    println!("   Token Symbol: {}", ft_symbol);
    println!("   Decimals: {}", ft_decimals);
    println!("   Total Supply: {}", ft_total_supply);
    println!();
    
    // Initialize testnet worker
    let worker = Worker::<Testnet>::testnet().await?;
    
    // Import parent account
    println!("🔑 Importing parent account...");
    let sk: SecretKey = parent_private_key.parse()?;
    let parent = worker.import_account(&parent_account_id.parse()?, &sk).await?;
    
    // Check parent account balance
    let parent_balance = parent.view_account().await?.balance;
    println!("✅ Parent account imported");
    println!("   Balance: {} NEAR", parent_balance);
    
    // Build the contract
    println!("\n📦 Building contract...");
    let output = std::process::Command::new("cargo")
        .args(&["build", "--release", "--target", "wasm32-unknown-unknown"])
        .current_dir("contracts/ft")
        .output()?;
    
    if !output.status.success() {
        anyhow::bail!("Failed to build contract: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    let wasm_path = "contracts/ft/target/wasm32-unknown-unknown/release/fungible_token.wasm";
    if !std::path::Path::new(wasm_path).exists() {
        anyhow::bail!("WASM file not found at {}. Make sure to build the contract first.", wasm_path);
    }
    
    let wasm = std::fs::read(wasm_path)?;
    println!("✅ Contract built successfully");
    println!("   WASM size: {} bytes", wasm.len());
    
    // Create subaccount
    println!("\n👶 Creating subaccount {}...", subaccount_id);
    
    let create_result = parent
        .create_subaccount(&subaccount_id)
        .initial_balance(NearToken::from_near(5)) // 5 NEAR initial balance
        .transact()
        .await?;
    
    let subaccount = create_result.into_result()?;
    println!("✅ Subaccount created successfully");
    
    // Deploy contract to subaccount
    println!("\n📤 Deploying contract to subaccount...");
    let deploy_result = subaccount.deploy(&wasm).await?;
    deploy_result.into_result()?;
    println!("✅ Contract deployed");
    
    // Initialize the contract
    println!("\n🎯 Initializing fungible token contract...");
    let init_args = json!({
        "owner_id": subaccount_id,
        "total_supply": ft_total_supply,
        "metadata": {
            "spec": "ft-1.0.0",
            "name": ft_name,
            "symbol": ft_symbol,
            "decimals": ft_decimals,
        }
    });
    
    let init_result = subaccount
        .call(subaccount.id(), "new")
        .args_json(init_args)
        .transact()
        .await?;
    init_result.into_result()?;
    
    println!("✅ Contract initialized successfully");
    
    // Verify deployment
    println!("\n🔍 Verifying deployment...");
    
    // Check metadata
    let metadata: serde_json::Value = subaccount
        .view(subaccount.id(), "ft_metadata")
        .await?
        .json()?;
    
    println!("📋 Token Metadata:");
    println!("   Name: {}", metadata["name"]);
    println!("   Symbol: {}", metadata["symbol"]);
    println!("   Decimals: {}", metadata["decimals"]);
    
    // Check total supply
    let total_supply: String = subaccount
        .view(subaccount.id(), "ft_total_supply")
        .await?
        .json()?;
    println!("   Total Supply: {}", total_supply);
    
    // Check owner balance
    let owner_balance: String = subaccount
        .view(subaccount.id(), "ft_balance_of")
        .args_json(json!({ "account_id": subaccount_id }))
        .await?
        .json()?;
    println!("   Owner Balance: {}", owner_balance);
    
    // Save deployment info
    println!("\n💾 Saving deployment information...");
    let deployment_info = format!(
        "# FT Contract Deployment Info\n\
        FT_CONTRACT_ID={}\n\
        PARENT_ACCOUNT={}\n\
        DEPLOYMENT_TIME={}\n\
        \n\
        # Contract Details\n\
        FT_NAME={}\n\
        FT_SYMBOL={}\n\
        FT_DECIMALS={}\n\
        FT_TOTAL_SUPPLY={}\n",
        subaccount_id,
        parent_account_id,
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
        ft_name,
        ft_symbol,
        ft_decimals,
        ft_total_supply
    );
    
    std::fs::write("deployment-info.env", deployment_info)?;
    println!("✅ Deployment info saved to deployment-info.env");
    
    println!("\n🎉 Deployment Complete!");
    println!("   Contract ID: {}", subaccount_id);
    println!("   Network: testnet");
    println!("\n   You can now interact with your token at: {}", subaccount_id);
    println!("   View on explorer: https://testnet.nearblocks.io/address/{}", subaccount_id);
    
    Ok(())
} 