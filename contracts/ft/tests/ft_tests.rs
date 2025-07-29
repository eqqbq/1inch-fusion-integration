use near_workspaces::{types::NearToken, Account, Contract};
use serde_json::json;

#[tokio::test]
async fn test_ft_total_supply() -> Result<(), Box<dyn std::error::Error>> {
    let worker = near_workspaces::sandbox().await?;
    let contract_wasm = near_workspaces::compile_project("../").await?;

    let contract = worker.dev_deploy(&contract_wasm).await?;

    // Initialize the contract
    let outcome = contract
        .call("new")
        .args_json(json!({
            "owner_id": contract.id(),
            "total_supply": "1000000000000000",
            "metadata": {
                "spec": "ft-1.0.0",
                "name": "Test Token",
                "symbol": "TEST",
                "decimals": 8
            }
        }))
        .transact()
        .await?;

    println!("outcome: {:?}", outcome);
    assert!(outcome.is_success());

    // Check total supply
    let total_supply: String = contract.view("ft_total_supply").await?.json()?;
    assert_eq!(total_supply, "1000000000000000");

    Ok(())
}

#[tokio::test]
async fn test_ft_transfer() -> Result<(), Box<dyn std::error::Error>> {
    let worker = near_workspaces::sandbox().await?;
    let contract_wasm = near_workspaces::compile_project("../").await?;

    let contract = worker.dev_deploy(&contract_wasm).await?;
    let alice = worker.dev_create_account().await?;
    let bob = worker.dev_create_account().await?;

    // Initialize the contract
    contract
        .call("new")
        .args_json(json!({
            "owner_id": alice.id(),
            "total_supply": "1000000000000000",
            "metadata": {
                "spec": "ft-1.0.0",
                "name": "Test Token",
                "symbol": "TEST",
                "decimals": 8
            }
        }))
        .transact()
        .await?;

    // Check Alice's initial balance
    let alice_balance: String = contract
        .view("ft_balance_of")
        .args_json(json!({ "account_id": alice.id() }))
        .await?
        .json()?;
    assert_eq!(alice_balance, "1000000000000000");

    // Register Bob's account for storage
    let outcome = bob
        .call(contract.id(), "storage_deposit")
        .args_json(json!({}))
        .deposit(NearToken::from_millinear(1))
        .transact()
        .await?;
    assert!(outcome.is_success());

    // Transfer tokens from Alice to Bob
    let outcome = alice
        .call(contract.id(), "ft_transfer")
        .args_json(json!({
            "receiver_id": bob.id(),
            "amount": "100000000000"
        }))
        .deposit(NearToken::from_yoctonear(1))
        .transact()
        .await?;
    assert!(outcome.is_success());

    // Check balances after transfer
    let alice_balance: String = contract
        .view("ft_balance_of")
        .args_json(json!({ "account_id": alice.id() }))
        .await?
        .json()?;
    assert_eq!(alice_balance, "999900000000000");

    let bob_balance: String = contract
        .view("ft_balance_of")
        .args_json(json!({ "account_id": bob.id() }))
        .await?
        .json()?;
    assert_eq!(bob_balance, "100000000000");

    Ok(())
}

#[tokio::test]
async fn test_ft_metadata() -> Result<(), Box<dyn std::error::Error>> {
    let worker = near_workspaces::sandbox().await?;
    let contract_wasm = near_workspaces::compile_project("../").await?;

    let contract = worker.dev_deploy(&contract_wasm).await?;

    // Initialize the contract
    contract
        .call("new")
        .args_json(json!({
            "owner_id": contract.id(),
            "total_supply": "1000000000000000",
            "metadata": {
                "spec": "ft-1.0.0",
                "name": "Test Token",
                "symbol": "TEST",
                "decimals": 8,
                "icon": "data:image/svg+xml,<svg></svg>"
            }
        }))
        .transact()
        .await?;

    // Check metadata
    let metadata: serde_json::Value = contract.view("ft_metadata").await?.json()?;
    assert_eq!(metadata["name"], "Test Token");
    assert_eq!(metadata["symbol"], "TEST");
    assert_eq!(metadata["decimals"], 8);
    assert_eq!(metadata["spec"], "ft-1.0.0");
    assert_eq!(metadata["icon"], "data:image/svg+xml,<svg></svg>");

    Ok(())
}

#[tokio::test]
async fn test_storage_management() -> Result<(), Box<dyn std::error::Error>> {
    let worker = near_workspaces::sandbox().await?;
    let contract_wasm = near_workspaces::compile_project("../").await?;

    let contract = worker.dev_deploy(&contract_wasm).await?;
    let alice = worker.dev_create_account().await?;

    // Initialize the contract
    contract
        .call("new")
        .args_json(json!({
            "owner_id": contract.id(),
            "total_supply": "1000000000000000",
            "metadata": {
                "spec": "ft-1.0.0",
                "name": "Test Token",
                "symbol": "TEST",
                "decimals": 8
            }
        }))
        .transact()
        .await?;

    // Check storage bounds
    let bounds: serde_json::Value = contract.view("storage_balance_bounds").await?.json()?;
    assert!(bounds["min"].is_string());

    // Check that Alice doesn't have storage initially
    let storage: serde_json::Value = contract
        .view("storage_balance_of")
        .args_json(json!({ "account_id": alice.id() }))
        .await?
        .json()?;
    assert!(storage.is_null());

    // Register Alice for storage
    let outcome = alice
        .call(contract.id(), "storage_deposit")
        .args_json(json!({}))
        .deposit(NearToken::from_millinear(1))
        .transact()
        .await?;
    assert!(outcome.is_success());

    // Check that Alice now has storage
    let storage: serde_json::Value = contract
        .view("storage_balance_of")
        .args_json(json!({ "account_id": alice.id() }))
        .await?
        .json()?;
    assert!(!storage.is_null());
    assert!(storage["total"].is_string());

    Ok(())
} 