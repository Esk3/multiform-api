use std::sync::Arc;

pub mod billett;
pub mod model;
pub mod person;

#[derive(Debug, sqlx::FromRow)]
pub struct Bestilling {
    pub id: i32,
}
pub async fn ny_bestilling(
    pool: Arc<sqlx::Pool<sqlx::Postgres>>,
) -> Result<Bestilling, sqlx::Error> {
    sqlx::query_as("insert into bestilling values (default) returning id")
        .fetch_one(&*pool)
        .await
}
