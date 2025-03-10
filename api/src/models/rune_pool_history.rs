use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr, TimestampSeconds};
use sqlx::FromRow;

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct RunePoolHistory {
    #[serde_as(as = "TimestampSeconds<String>")]
    pub start_time: DateTime<Utc>,

    #[serde_as(as = "TimestampSeconds<String>")]
    pub end_time: DateTime<Utc>,

    #[serde_as(as = "DisplayFromStr")]
    pub count: i64,

    #[serde_as(as = "DisplayFromStr")]
    pub units: i64,
}
