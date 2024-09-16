use std::sync::Arc;

use poem_openapi::{param::Path, payload::PlainText, ApiResponse, OpenApi};

use super::router_args::Args;

mod get_billett;
mod lagre_billett;
pub mod model;

pub struct BilletApi {
    pub pool: Arc<sqlx::Pool<sqlx::Postgres>>,
}

#[OpenApi]
impl BilletApi {
    #[oai(path = "/billett", method = "get")]
    async fn index(&self) -> PlainText<String> {
        PlainText("billett".to_string())
    }
    #[oai(path = "/billett/:id", method = "get")]
    async fn get_billett(&self, id: Path<i32>) -> GetBillettResponse {
        dbg!("here");
        match get_billett(*id, self.pool.clone()).await {
            Ok(res) => GetBillettResponse::Ok(PlainText(res)),
            Err(_) => GetBillettResponse::NotFound,
        }
    }
}

#[derive(Debug, ApiResponse)]
enum GetBillettResponse {
    #[oai(status = 200)]
    Ok(PlainText<String>),
    #[oai(status = 404)]
    NotFound,
}

async fn get_billett(id: i32, pool: Arc<sqlx::PgPool>) -> Result<String, sqlx::Error> {
    dbg!("quering billett med id ", id);
    let result: model::Billett = dbg!(dbg!(
        sqlx::query_as("select bestillings_id, fra_iata_code, til_iata_code, status::text, billett_type::text, timestamp::text from billett where bestillings_id = $1")
            .bind(id)
            .fetch_one(&*pool)
            .await
    )?);
    Ok(format!(
        "
        fra: {}
        til: {}
        status: {}
        billett type: {}
            ",
        result.fra_iata_code, result.til_iata_code, result.status, result.billett_type
    ))
}
