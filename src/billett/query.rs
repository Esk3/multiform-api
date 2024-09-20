use std::sync::Arc;

use super::model::{self, Billett};

pub struct BillettQuery {
    pool: Arc<sqlx::Pool<sqlx::Postgres>>,
}
impl BillettQuery {
    pub fn new(pool: Arc<sqlx::Pool<sqlx::Postgres>>) -> Self {
        Self { pool }
    }

    pub async fn get_billett_by_id(&self, id: i32) -> Result<Option<Billett>, sqlx::Error> {
        sqlx::query_as(
            "select bestillings_id, fra_iata_code, til_iata_code,
                status::text, billett_type::text, timestamp::text
                from billett
                where bestillings_id = $1",
        )
        .bind(id)
        .fetch_optional(&*self.pool)
        .await
    }
    pub async fn insert_billet(
        &self,
        model::BillettForm {
            reise_id,
            person_id,
            bekreftet: _,
            status,
            billett_type,
        }: &model::BillettForm,
    ) -> Result<model::Billett, sqlx::Error> {
        sqlx::query_as(
            "insert into billett (reise_id, person_id, status, billett_type)
            values ($1, $2, $3, $4, $5)
            returning bestillings_id
            returning *",
        )
        .bind(reise_id)
        .bind(person_id)
        .bind(status)
        .bind(billett_type)
        .fetch_one(&*self.pool)
        .await
    }
}
