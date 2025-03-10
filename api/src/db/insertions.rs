use crate::models::{DepthPriceHistory, EarningsHistory, RunePoolHistory, SwapsHistory};
use chrono::{DateTime, Utc};
use sqlx::PgPool;

pub async fn get_last_end_time(
    pool: &PgPool,
    table: &str,
) -> Result<Option<DateTime<Utc>>, sqlx::Error> {
    let last_end_time: Option<DateTime<Utc>> =
        sqlx::query_scalar(&format!("SELECT max(end_time) FROM {}", table))
            .fetch_optional(pool)
            .await?;
    Ok(last_end_time)
}

pub async fn insert_depth_price_history(
    pool: &PgPool,
    data: &Vec<DepthPriceHistory>,
) -> Result<(), sqlx::Error> {
    println!("Inserting depth price history...\n");
    for item in data {
        sqlx::query(
            "INSERT INTO depth_price_history (start_time, end_time, asset_depth, rune_depth, asset_price, asset_price_usd, liquidity_units, members_count, synth_units, synth_supply, units, luvi) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
        )
        .bind(item.start_time)
        .bind(item.end_time)
        .bind(item.asset_depth)
        .bind(item.rune_depth)
        .bind(&item.asset_price)
        .bind(&item.asset_price_usd)
        .bind(item.liquidity_units)
        .bind(item.members_count)
        .bind(item.synth_units)
        .bind(item.synth_supply)
        .bind(item.units)
        .bind(&item.luvi)
        .execute(pool)
        .await?;
    }
    Ok(())
}

pub async fn insert_earnings_history(
    pool: &PgPool,
    data: &Vec<EarningsHistory>,
) -> Result<(), sqlx::Error> {
    println!("Inserting earnings history...\n");
    for item in data {
        let earnings_history_id: (i32,) = sqlx::query_as(
            "INSERT INTO earnings_history (start_time, end_time, liquidity_fees, block_rewards, earnings, bonding_earnings, liquidity_earnings, avg_node_count, rune_price_usd) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING id",
        )
        .bind(item.start_time)
        .bind(item.end_time)
        .bind(item.liquidity_fees)
        .bind(item.block_rewards)
        .bind(item.earnings)
        .bind(item.bonding_earnings)
        .bind(item.liquidity_earnings)
        .bind(&item.avg_node_count)
        .bind(&item.rune_price_usd)
        .fetch_one(pool)
        .await?;

        for pool_earning in &item.pools {
            sqlx::query(
                "INSERT INTO pool_earnings (earnings_history_id, pool, asset_liquidity_fees, rune_liquidity_fees, total_liquidity_fees_rune, saver_earning, rewards, earnings) 
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            )
            .bind(earnings_history_id.0)
            .bind(&pool_earning.pool)
            .bind(pool_earning.asset_liquidity_fees)
            .bind(pool_earning.rune_liquidity_fees)
            .bind(pool_earning.total_liquidity_fees_rune)
            .bind(pool_earning.saver_earning)
            .bind(pool_earning.rewards)
            .bind(pool_earning.earnings)
            .execute(pool)
            .await?;
        }
    }
    Ok(())
}

pub async fn insert_rune_pool_history(
    pool: &PgPool,
    data: &Vec<RunePoolHistory>,
) -> Result<(), sqlx::Error> {
    println!("Inserting rune pool history...\n");
    for item in data {
        sqlx::query(
            "INSERT INTO rune_pool_history (start_time, end_time, count, units) 
             VALUES ($1, $2, $3, $4)",
        )
        .bind(item.start_time)
        .bind(item.end_time)
        .bind(item.count)
        .bind(item.units)
        .execute(pool)
        .await?;
    }
    Ok(())
}

pub async fn insert_swaps_history(
    pool: &PgPool,
    data: &Vec<SwapsHistory>,
) -> Result<(), sqlx::Error> {
    println!("Inserting swaps history...\n");
    for item in data {
        sqlx::query(
            "INSERT INTO swaps_history (
                start_time, end_time, 
                to_asset_count, to_rune_count, to_trade_count, from_trade_count, 
                synth_mint_count, synth_redeem_count, total_count,
                to_asset_volume, to_rune_volume, to_trade_volume, from_trade_volume,
                synth_mint_volume, synth_redeem_volume, total_volume,
                to_asset_volume_usd, to_rune_volume_usd, to_trade_volume_usd, from_trade_volume_usd,
                synth_mint_volume_usd, synth_redeem_volume_usd, total_volume_usd,
                to_asset_fees, to_rune_fees, to_trade_fees, from_trade_fees,
                synth_mint_fees, synth_redeem_fees, total_fees,
                to_asset_average_slip, to_rune_average_slip, to_trade_average_slip, from_trade_average_slip,
                synth_mint_average_slip, synth_redeem_average_slip, average_slip,
                rune_price_usd
            ) VALUES (
                $1, $2, 
                $3, $4, $5, $6, 
                $7, $8, $9, 
                $10, $11, $12, $13, 
                $14, $15, $16, 
                $17, $18, $19, $20, 
                $21, $22, $23, 
                $24, $25, $26, $27, 
                $28, $29, $30, 
                $31, $32, $33, $34, 
                $35, $36, $37, 
                $38
            )"
        )
        .bind(item.start_time)
        .bind(item.end_time)
        .bind(item.to_asset_count)
        .bind(item.to_rune_count)
        .bind(item.to_trade_count)
        .bind(item.from_trade_count)
        .bind(item.synth_mint_count)
        .bind(item.synth_redeem_count)
        .bind(item.total_count)
        .bind(item.to_asset_volume)
        .bind(item.to_rune_volume)
        .bind(item.to_trade_volume)
        .bind(item.from_trade_volume)
        .bind(item.synth_mint_volume)
        .bind(item.synth_redeem_volume)
        .bind(item.total_volume)
        .bind(item.to_asset_volume_usd)
        .bind(item.to_rune_volume_usd)
        .bind(item.to_trade_volume_usd)
        .bind(item.from_trade_volume_usd)
        .bind(item.synth_mint_volume_usd)
        .bind(item.synth_redeem_volume_usd)
        .bind(item.total_volume_usd)
        .bind(item.to_asset_fees)
        .bind(item.to_rune_fees)
        .bind(item.to_trade_fees)
        .bind(item.from_trade_fees)
        .bind(item.synth_mint_fees)
        .bind(item.synth_redeem_fees)
        .bind(item.total_fees)
        .bind(&item.to_asset_average_slip)
        .bind(&item.to_rune_average_slip)
        .bind(&item.to_trade_average_slip)
        .bind(&item.from_trade_average_slip)
        .bind(&item.synth_mint_average_slip)
        .bind(&item.synth_redeem_average_slip)
        .bind(&item.average_slip)
        .bind(&item.rune_price_usd)
        .execute(pool)
        .await?;
    }
    Ok(())
}
