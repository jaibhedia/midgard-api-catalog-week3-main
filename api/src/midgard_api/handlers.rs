use super::{interface::Interface, Params};
use crate::models::{DepthPriceHistory, EarningsHistory, RunePoolHistory, SwapsHistory};
use anyhow::Result;

pub async fn fetch_depth_price_history(params: Params) -> Result<Vec<DepthPriceHistory>> {
    let api_interface = Interface::new("depths/BTC.BTC".to_string(), params);
    println!("Fetching depth price history...");
    api_interface.fetch_data().await
}

pub async fn fetch_earnings_history(params: Params) -> Result<Vec<EarningsHistory>> {
    let api_interface = Interface::new("earnings".to_string(), params);
    println!("Fetching earnings history...");
    api_interface.fetch_data().await
}

pub async fn fetch_rune_pool_history(params: Params) -> Result<Vec<RunePoolHistory>> {
    let api_interface = Interface::new("runepool".to_string(), params);
    println!("Fetching rune pool history...");
    api_interface.fetch_data().await
}

pub async fn fetch_swaps_history(params: Params) -> Result<Vec<SwapsHistory>> {
    let api_interface = Interface::new("swaps".to_string(), params);
    println!("Fetching swaps history...");
    api_interface.fetch_data().await
}
