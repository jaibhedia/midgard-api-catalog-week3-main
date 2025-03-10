use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr, TimestampSeconds};
use sqlx::{types::BigDecimal, FromRow};

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct DepthPriceHistory {
    #[serde_as(as = "TimestampSeconds<String>")]
    pub start_time: DateTime<Utc>,

    #[serde_as(as = "TimestampSeconds<String>")]
    pub end_time: DateTime<Utc>,

    #[serde_as(as = "DisplayFromStr")]
    pub asset_depth: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub rune_depth: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub asset_price: BigDecimal,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "assetPriceUSD")]
    pub asset_price_usd: BigDecimal,

    #[serde_as(as = "DisplayFromStr")]
    pub liquidity_units: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub members_count: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub synth_units: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub synth_supply: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub units: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub luvi: BigDecimal,
}
