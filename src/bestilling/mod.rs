use std::sync::Arc;

mod bestilling_id;
pub mod billett;
mod model;
pub mod person;
pub mod router_args;

#[derive(Debug, sqlx::FromRow)]
pub struct Bestilling {
    id: i32,
}
pub async fn ny_bestilling(
    pool: Arc<sqlx::Pool<sqlx::Postgres>>,
) -> Result<Bestilling, sqlx::Error> {
    sqlx::query_as("insert into bestilling values (default) returning id")
        .fetch_one(&*pool)
        .await
}
