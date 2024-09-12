use std::sync::Arc;

use super::model::Billett;

async fn lagre_billett(billet: Billett, pool: Arc<sqlx::PgPool>) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
    sqlx::query(
        "insert into billett 
        (bestillings_id, fra_iata_code, til_iata_code, status, billett_type)
        values
        ($1, $2, $3, $4, $5)",
    )
    .bind(billet.bestillings_id)
    .bind(billet.fra_iata_code)
    .bind(billet.til_iata_code)
    .bind(billet.status)
    .bind(billet.billett_type)
    .execute(&*pool)
    .await
}
