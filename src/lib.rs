use near_workspaces::{Account, Worker};
use near_workspaces::network::{Mainnet, Testnet};
use std::env;
use anyhow::Result;

pub async fn init_mainnet_worker() -> Result<Worker<Mainnet>> {
    let worker = near_workspaces::mainnet().await?;
    Ok(worker)
}

pub async fn init_testnet_worker() -> Result<Worker<Testnet>> {
    let worker = near_workspaces::testnet().await?;
    Ok(worker)
}

pub fn get_env_var(var_name: &str) -> Result<String> {
    env::var(var_name).map_err(|_| anyhow::anyhow!("{} not set in environment", var_name))
}

pub async fn deploy_ft_contract(
    account: &Account,
    wasm_path: &str,
    owner_id: &str,
    total_supply: &str,
    metadata_name: &str,
    metadata_symbol: &str,
    metadata_decimals: u8,
) -> Result<()> {
    let wasm = std::fs::read(wasm_path)?;
    
    // Deploy contract
    let result = account.deploy(&wasm).await?;
    result.into_result()?;
    
    // Initialize contract
    let init_args = serde_json::json!({
        "owner_id": owner_id,
        "total_supply": total_supply,
        "metadata": {
            "spec": "ft-1.0.0",
            "name": metadata_name,
            "symbol": metadata_symbol,
            "decimals": metadata_decimals,
        }
    });
    
    let result = account.call(account.id(), "new")
        .args_json(init_args)
        .transact()
        .await?;
    result.into_result()?;
    
    println!("✅ FT Contract deployed and initialized at: {}", account.id());
    Ok(())
}
