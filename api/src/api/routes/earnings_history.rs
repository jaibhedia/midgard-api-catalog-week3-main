use crate::utils::parse_date_to_utc;
use crate::{api::ApiParams, models::ApiEarningsHistory};
use axum::{
    extract::{Query, State},
    response::Json,
};
use reqwest::StatusCode;
use serde_json::Value;
use sqlx::{PgPool, Row};
use std::sync::Arc;

pub async fn get_earnings_history(
    State(db_pool): State<Arc<PgPool>>,
    Query(params): Query<ApiParams>,
) -> Result<Json<Vec<ApiEarningsHistory>>, (StatusCode, String)> {
    let mut query = String::new();

    // Calculate interval
    let allowed_intervals = vec!["hour", "day", "week", "month"];
    let interval = match &params.interval {
        Some(interval) => {
            if !allowed_intervals.contains(&interval.as_str()) {
                return Err((
                    StatusCode::BAD_REQUEST,
                    format!(
                        "Invalid interval provided. Allowed intervals: {:?}",
                        allowed_intervals
                    ),
                ));
            }
            interval
        }
        None => "",
    };

    // Start building the query
    let join_clause = r#" 
            e.start_time,
            e.end_time,
            e.liquidity_fees,
            e.block_rewards,
            e.earnings,
            e.bonding_earnings,
            e.liquidity_earnings,
            e.avg_node_count,
            e.rune_price_usd,
            json_agg(json_build_object(
                'pool', pe.pool,
                'asset_liquidity_fees', pe.asset_liquidity_fees,
                'rune_liquidity_fees', pe.rune_liquidity_fees,
                'total_liquidity_fees_rune', pe.total_liquidity_fees_rune,
                'saver_earning', pe.saver_earning,
                'rewards', pe.rewards,
                'earnings', pe.earnings
            )) AS pools
        FROM earnings_history e
        LEFT JOIN pool_earnings pe ON e.id = pe.earnings_history_id
        GROUP BY e.start_time, e.end_time, e.liquidity_fees, e.block_rewards, e.earnings, e.bonding_earnings, e.liquidity_earnings, e.avg_node_count, e.rune_price_usd
    "#;

    let select_clause = if interval.is_empty() {
        format!("SELECT {}", join_clause)
    } else {
        format!(
            "SELECT DISTINCT ON (date_trunc('{}', start_time)) {}",
            interval, join_clause
        )
    };
    query.push_str(&select_clause);

    // Parse the dates if provided
    if let Some(date_range) = &params.date_range {
        let dates: Vec<&str> = date_range.split(',').collect();

        let mut start_date = None;
        let mut end_date = None;

        if !dates[0].is_empty() {
            start_date = parse_date_to_utc(dates[0]);
        }

        if dates.len() > 1 && !dates[1].is_empty() {
            end_date = parse_date_to_utc(dates[1]);
        }

        if start_date.is_some() || end_date.is_some() {
            query.push_str(" HAVING ");
            if let Some(start) = start_date {
                query.push_str(&format!("start_time >= '{}'", start.to_rfc3339()));
            }
            if let Some(end) = end_date {
                if start_date.is_some() {
                    query.push_str(" AND ");
                }
                query.push_str(&format!("end_time <= '{}'", end.to_rfc3339()));
            }
        }
    }

    // Sorting
    let order = params.order.unwrap_or("ASC".to_string());
    let order_clause = match (interval, &params.sort_by) {
        ("", None) => " ORDER BY start_time".to_string(),
        ("", Some(sort_by)) => format!(" ORDER BY {} {}", sort_by, order),
        (interval, None) => format!(
            " ORDER BY date_trunc('{}', start_time), start_time {}",
            interval, order
        ),
        (interval, Some(sort_by)) => format!(
            " ORDER BY date_trunc('{}', start_time), start_time, {} {}",
            interval, sort_by, order
        ),
    };
    query.push_str(&order_clause);

    // Pagination
    let limit = params.limit.unwrap_or(10);
    let page = params.page.unwrap_or(1);
    let offset = (page - 1) * limit;

    query.push_str(&format!(" LIMIT {}", limit));
    query.push_str(&format!(" OFFSET {}", offset));

    // Fetch
    let rows = sqlx::query(&query)
        .fetch_all(&*db_pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let earnings_history = rows
        .into_iter()
        .map(|row| {
            let start_time: chrono::DateTime<chrono::Utc> = row.get("start_time");
            let end_time: chrono::DateTime<chrono::Utc> = row.get("end_time");
            let liquidity_fees: i64 = row.get("liquidity_fees");
            let block_rewards: i64 = row.get("block_rewards");
            let earnings: i64 = row.get("earnings");
            let bonding_earnings: i64 = row.get("bonding_earnings");
            let liquidity_earnings: i64 = row.get("liquidity_earnings");
            let avg_node_count: sqlx::types::BigDecimal = row.get("avg_node_count");
            let rune_price_usd: sqlx::types::BigDecimal = row.get("rune_price_usd");

            // Parse pools JSON
            let pools_json = row.get("pools");
            let pools: Vec<Value> = serde_json::from_value(pools_json).unwrap_or_else(|_| vec![]);

            ApiEarningsHistory {
                start_time,
                end_time,
                liquidity_fees,
                block_rewards,
                earnings,
                bonding_earnings,
                liquidity_earnings,
                avg_node_count,
                rune_price_usd,
                pools,
            }
        })
        .collect::<Vec<ApiEarningsHistory>>();

    Ok(Json(earnings_history))
}
