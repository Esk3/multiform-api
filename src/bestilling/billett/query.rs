use std::sync::Arc;

use super::model::Billett;

pub struct BillettQuery {
    pool: Arc<sqlx::Pool<sqlx::Postgres>>,
}
impl BillettQuery {
    pub fn new(pool: Arc<sqlx::Pool<sqlx::Postgres>>) -> Self {
        Self { pool }
    }

    pub async fn get_billett_by_id(&self, id: i32) -> Result<Option<Billett>, sqlx::Error> {
        dbg!("quering billett med id ", id);
        dbg!(dbg!(
        sqlx::query_as("select bestillings_id, fra_iata_code, til_iata_code, status::text, billett_type::text, timestamp::text from billett where bestillings_id = $1")
            .bind(id)
            .fetch_optional(&*self.pool)
            .await
    ))
    }
}
