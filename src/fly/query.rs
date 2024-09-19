use std::sync::Arc;

use super::model;

pub struct FlyQuery {
    pool: Arc<sqlx::Pool<sqlx::Postgres>>,
}

impl FlyQuery {
    pub fn new(pool: Arc<sqlx::Pool<sqlx::Postgres>>) -> Self {
        Self { pool }
    }
    pub async fn get_fly(&self) -> Result<Vec<model::Fly>, sqlx::Error> {
        sqlx::query_as(
            "select *
            from fly",
        )
        .fetch_all(&*self.pool)
        .await
    }
    pub async fn get_fly_by_id(&self, id: i32) -> Result<Option<model::Fly>, sqlx::Error> {
        sqlx::query_as(
            "select *
            from fly
            where fly_id = $1",
        )
        .bind(id)
        .fetch_optional(&*self.pool)
        .await
    }
}
