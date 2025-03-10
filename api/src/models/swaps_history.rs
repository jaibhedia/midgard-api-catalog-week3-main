use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr, TimestampSeconds};
use sqlx::{types::BigDecimal, FromRow};

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct SwapsHistory {
    #[serde_as(as = "TimestampSeconds<String>")]
    pub start_time: DateTime<Utc>,

    #[serde_as(as = "TimestampSeconds<String>")]
    pub end_time: DateTime<Utc>,

    #[serde_as(as = "DisplayFromStr")]
    pub to_asset_count: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub to_rune_count: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub to_trade_count: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub from_trade_count: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub synth_mint_count: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub synth_redeem_count: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub total_count: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub to_asset_volume: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub to_rune_volume: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub to_trade_volume: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub from_trade_volume: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub synth_mint_volume: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub synth_redeem_volume: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub total_volume: i64,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "toAssetVolumeUSD")]
    pub to_asset_volume_usd: i64,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "toRuneVolumeUSD")]
    pub to_rune_volume_usd: i64,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "toTradeVolumeUSD")]
    pub to_trade_volume_usd: i64,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "fromTradeVolumeUSD")]
    pub from_trade_volume_usd: i64,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "synthMintVolumeUSD")]
    pub synth_mint_volume_usd: i64,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "synthRedeemVolumeUSD")]
    pub synth_redeem_volume_usd: i64,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "totalVolumeUSD")]
    pub total_volume_usd: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub to_asset_fees: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub to_rune_fees: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub to_trade_fees: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub from_trade_fees: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub synth_mint_fees: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub synth_redeem_fees: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub total_fees: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub to_asset_average_slip: BigDecimal,

    #[serde_as(as = "DisplayFromStr")]
    pub to_rune_average_slip: BigDecimal,

    #[serde_as(as = "DisplayFromStr")]
    pub to_trade_average_slip: BigDecimal,

    #[serde_as(as = "DisplayFromStr")]
    pub from_trade_average_slip: BigDecimal,

    #[serde_as(as = "DisplayFromStr")]
    pub synth_mint_average_slip: BigDecimal,

    #[serde_as(as = "DisplayFromStr")]
    pub synth_redeem_average_slip: BigDecimal,

    #[serde_as(as = "DisplayFromStr")]
    pub average_slip: BigDecimal,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "runePriceUSD")]
    pub rune_price_usd: BigDecimal,
}
