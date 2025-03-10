use api::routes::{
    docs, get_depth_price_history, get_earnings_history, get_rune_pool_history, get_swaps_history,
};
use axum::{routing::get, Router};
use db::init_db;
use dotenv::dotenv;
use std::{env, sync::Arc};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

mod api;
mod db;
mod midgard_api;
mod models;
mod populate_db;
mod scheduler;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();
    utils::init_tracing();

    // Initialize database pool
    let db_pool = match init_db().await {
        Ok(db) => {
            println!("\nConnected to database!\n");
            Arc::new(db)
        }
        Err(error) => {
            eprintln!("Failed to initialize database | {error}");
            return;
        }
    };

    // Start scheduler
    let scheduler_pool = Arc::clone(&db_pool);
    tokio::spawn(async move {
        scheduler::start_scheduler(scheduler_pool).await;
    });

    // Start server
    let app = Router::new()
        .route("/", get(docs))
        .route("/history/depth", get(get_depth_price_history))
        .route("/history/earnings", get(get_earnings_history))
        .route("/history/rune-pool", get(get_rune_pool_history))
        .route("/history/swaps", get(get_swaps_history))
        .with_state(Arc::clone(&db_pool))
        .layer(tower_http::catch_panic::CatchPanicLayer::new())
        .layer(TraceLayer::new_for_http());

    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Could not create listener");

    println!("Server running on port {}", port);

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
