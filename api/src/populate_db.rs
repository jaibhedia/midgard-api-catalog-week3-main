use crate::db::insertions::{self, get_last_end_time};
use crate::midgard_api::{self, handlers};
use crate::utils::get_truncated_now;
use chrono::{DateTime, Duration, Utc};
use sqlx::PgPool;

fn midgard_params(start_time: DateTime<Utc>) -> midgard_api::Params {
    midgard_api::Params {
        interval: "hour".to_string(),
        from: start_time,
        count: 400,
    }
}

pub async fn populate_db(db_pool: &PgPool) {
    println!("\nPopulating database...");

    // Calculate last timestamp in database
    let default_start_time: DateTime<Utc> = get_truncated_now() - Duration::days(90);
    let mut last_end_time = match get_last_end_time(db_pool, "depth_price_history").await {
        Ok(time) => time.unwrap_or(default_start_time),
        Err(error) => {
            eprintln!("Failed to get last end time | {error}");
            default_start_time
        }
    };
    let mut current_iteration: u8 = 1;

    // Perform fetch and insert operations until last_end_time is within the last hour
    while last_end_time <= get_truncated_now() - Duration::hours(1) {
        let params = midgard_params(last_end_time);

        println!(
            "\n\n------------Iteration: {} | From: {}------------\n",
            current_iteration, &params.from
        );

        // Fetch and insert depth price history
        let depth_price_history = match handlers::fetch_depth_price_history(params.clone()).await {
            Ok(data) => data,
            Err(error) => {
                eprintln!("Failed to fetch depth price history | {error}");
                return;
            }
        };
        match insertions::insert_depth_price_history(db_pool, &depth_price_history).await {
            Ok(data) => data,
            Err(error) => {
                eprintln!("Failed to insert depth price history: {error}");
                return;
            }
        };

        // Fetch and insert earnings history
        let earnings_history = match handlers::fetch_earnings_history(params.clone()).await {
            Ok(data) => data,
            Err(error) => {
                eprintln!("Failed to fetch earnings history | {error}");
                return;
            }
        };
        match insertions::insert_earnings_history(db_pool, &earnings_history).await {
            Ok(data) => data,
            Err(error) => {
                eprintln!("Failed to insert earnings history: {error}");
                return;
            }
        };

        // Fetch and insert rune pool history
        let rune_pool_history = match handlers::fetch_rune_pool_history(params.clone()).await {
            Ok(data) => data,
            Err(error) => {
                eprintln!("Failed to fetch rune pool history | {error}");
                return;
            }
        };
        match insertions::insert_rune_pool_history(db_pool, &rune_pool_history).await {
            Ok(data) => data,
            Err(error) => {
                eprintln!("Failed to insert rune pool history: {error}");
                return;
            }
        };

        // Fetch and insert swaps history
        let swaps_history = match handlers::fetch_swaps_history(params.clone()).await {
            Ok(data) => data,
            Err(error) => {
                eprintln!("Failed to fetch swaps history | {error}");
                return;
            }
        };
        match insertions::insert_swaps_history(db_pool, &swaps_history).await {
            Ok(data) => data,
            Err(error) => {
                eprintln!("Failed to insert swaps history: {error}");
            }
        };

        // Update last_end_time
        last_end_time = match get_last_end_time(db_pool, "depth_price_history").await {
            Ok(time) => time.unwrap_or(default_start_time),
            Err(error) => {
                eprintln!("Failed to get last end time | {error}");
                return;
            }
        };
        current_iteration += 1;
    }
}
