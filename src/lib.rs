use std::env;
use anyhow::Result;

/// Get an environment variable or return an error with a helpful message
pub fn get_env_var(var_name: &str) -> Result<String> {
    env::var(var_name).map_err(|_| anyhow::anyhow!("{} not set in environment", var_name))
}

/// Format a token amount with proper decimals for display
pub fn format_token_amount(raw_amount: &str, decimals: u8) -> Result<String> {
    let amount = raw_amount.parse::<f64>()?;
    let divisor = 10f64.powi(decimals as i32);
    let formatted = amount / divisor;
    Ok(format!("{:.2}", formatted))
}
