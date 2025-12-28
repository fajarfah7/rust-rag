use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn new_pg_pool(url: &str) -> PgPool {
    PgPoolOptions::new()
    .max_connections(5)
    .connect(url)
    .await
    .unwrap()
}
