use crate::{
    api::{get_history, ApiParams},
    models::DepthPriceHistory,
};
use axum::{
    extract::{Query, State},
    response::Json,
};
use reqwest::StatusCode;
use sqlx::PgPool;
use std::sync::Arc;

pub async fn get_depth_price_history(
    state: State<Arc<PgPool>>,
    params: Query<ApiParams>,
) -> Result<Json<Vec<DepthPriceHistory>>, (StatusCode, String)> {
    get_history::<DepthPriceHistory>(state, params, "depth_price_history").await
}
