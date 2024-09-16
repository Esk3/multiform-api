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
            bestillings_id,
            fra_iata_code,
            til_iata_code,
            status,
            billett_type,
        }: &model::BillettForm,
    ) -> Result<sqlx::postgres::PgRow, sqlx::Error> {
        sqlx::query(
            "insert into billett (bestillings_id, fra_iata_code, til_iata_code, status, billett_type)
            values ($1, $2, $3, $4, $5)
            returning bestillings_id"
        ).bind(*bestillings_id)
            .bind(fra_iata_code)
            .bind(til_iata_code)
            .bind(status)
            .bind(billett_type)
            .fetch_one(&*self.pool)
            .await
    }
}
