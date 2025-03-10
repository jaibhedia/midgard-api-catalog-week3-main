pub mod routes;

use crate::utils::parse_date_to_utc;
use axum::{
    extract::{Query, State},
    response::Json,
};
use reqwest::StatusCode;
use sqlx::{FromRow, PgPool, Postgres, QueryBuilder};
use std::sync::Arc;

#[derive(serde::Deserialize)]
pub struct ApiParams {
    interval: Option<String>,
    date_range: Option<String>,
    sort_by: Option<String>,
    order: Option<String>,
    limit: Option<i64>,
    page: Option<i64>,
}

pub async fn get_history<T>(
    State(db_pool): State<Arc<PgPool>>,
    Query(params): Query<ApiParams>,
    table: &str,
) -> Result<Json<Vec<T>>, (StatusCode, String)>
where
    T: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Send + Sync + Unpin + 'static,
{
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
    let select_clause = if interval.is_empty() {
        format!("SELECT * FROM {}", table)
    } else {
        format!(
            "SELECT DISTINCT ON (date_trunc('{}', start_time)) * FROM {}",
            interval, table
        )
    };
    let mut query: QueryBuilder<Postgres> = QueryBuilder::new(select_clause);

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
            query.push(" WHERE ");
            if let Some(start) = start_date {
                query.push("start_time >= ").push_bind(start);
            }
            if let Some(end) = end_date {
                if start_date.is_some() {
                    query.push(" AND ");
                }
                query.push("end_time <= ").push_bind(end);
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
    query.push(order_clause);

    // Pagination
    let limit = params.limit.unwrap_or(10);
    let page = params.page.unwrap_or(1);
    let offset = (page - 1) * limit;

    query.push(" LIMIT ").push(limit);
    query.push(" OFFSET ").push(offset);

    // Final query and fetch
    let sql = query.build_query_as::<T>();

    let records = sql
        .fetch_all(&*db_pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(records))
}
