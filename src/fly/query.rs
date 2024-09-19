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
    pub async fn create_fly(&self, form: model::FlyForm) -> Result<model::Fly, sqlx::Error> {
        sqlx::query_as(
            "insert into fly (luxus_seter, flex_seter, billig_seter)
            values ($1, $2, $3)",
        )
        .bind(form.luxus_seter)
        .bind(form.flex_seter)
        .bind(form.billig_seter)
        .fetch_one(&*self.pool)
        .await
    }
}
