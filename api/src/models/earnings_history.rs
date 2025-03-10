use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{serde_as, DisplayFromStr, TimestampSeconds};
use sqlx::{types::BigDecimal, FromRow};

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct EarningsHistory {
    #[serde_as(as = "TimestampSeconds<String>")]
    pub start_time: DateTime<Utc>,

    #[serde_as(as = "TimestampSeconds<String>")]
    pub end_time: DateTime<Utc>,

    #[serde_as(as = "DisplayFromStr")]
    pub liquidity_fees: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub block_rewards: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub earnings: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub bonding_earnings: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub liquidity_earnings: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub avg_node_count: BigDecimal,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "runePriceUSD")]
    pub rune_price_usd: BigDecimal,

    pub pools: Vec<PoolEarnings>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct PoolEarnings {
    pub pool: String,

    #[serde_as(as = "DisplayFromStr")]
    pub asset_liquidity_fees: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub rune_liquidity_fees: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub total_liquidity_fees_rune: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub saver_earning: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub rewards: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub earnings: i64,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ApiEarningsHistory {
    #[serde_as(as = "TimestampSeconds<String>")]
    pub start_time: DateTime<Utc>,

    #[serde_as(as = "TimestampSeconds<String>")]
    pub end_time: DateTime<Utc>,

    #[serde_as(as = "DisplayFromStr")]
    pub liquidity_fees: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub block_rewards: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub earnings: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub bonding_earnings: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub liquidity_earnings: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub avg_node_count: BigDecimal,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "runePriceUSD")]
    pub rune_price_usd: BigDecimal,

    pub pools: Vec<Value>,
}
