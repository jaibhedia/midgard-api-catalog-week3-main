use crate::populate_db::populate_db;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::time::{self, Duration};

pub async fn start_scheduler(db_pool: Arc<PgPool>) {
    let mut interval = time::interval(Duration::from_secs(120));
    loop {
        interval.tick().await;
        populate_db(&db_pool).await;
    }
}
