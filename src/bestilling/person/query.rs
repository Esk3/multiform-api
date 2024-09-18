use std::sync::Arc;

use super::model;

pub struct PersonQuery {
    pool: Arc<sqlx::Pool<sqlx::Postgres>>,
}

impl PersonQuery {
    pub fn new(pool: Arc<sqlx::Pool<sqlx::Postgres>>) -> Self {
        Self { pool }
    }
    pub async fn get_fra_id(&self, id: i32) -> Result<model::Person, sqlx::Error> {
        sqlx::query_as(
            "select *
            from person
            where id = $1",
        )
        .bind(id)
        .fetch_one(&*self.pool)
        .await
    }
    pub async fn get_fra_første_navn(&self) {}
}
