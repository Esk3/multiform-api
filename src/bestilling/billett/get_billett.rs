use std::sync::Arc;

use super::model::Billett;

async fn get_billett(bestillings_id: i32, pool: Arc<sqlx::PgPool>) -> Result<Billett, sqlx::Error> {
    sqlx::query_as(
        "select * from billett
        where bestillings_id = $1",
    )
    .bind(bestillings_id)
    .fetch_one(&*pool)
    .await
}
