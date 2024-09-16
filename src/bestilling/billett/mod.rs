use poem::{http::HeaderMap, middleware::SetHeader, EndpointExt, IntoResponse};
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, sync::Arc};

use poem::Endpoint;
use poem_openapi::{
    param::{Cookie, Header, Path},
    payload::PlainText,
    ApiRequest, ApiResponse, OpenApi, SecurityScheme,
};

use super::bestilling_id;

mod get_billett;
mod lagre_billett;
pub mod model;

#[derive(ApiResponse)]
enum IndexResponse {
    #[oai(status = 200)]
    Ok(PlainText<String>, #[oai(header = "Set-Cookie")] String),
}

pub struct BilletApi {
    pub pool: Arc<sqlx::Pool<sqlx::Postgres>>,
}

#[OpenApi(prefix_path = "/v1/billett")]
impl BilletApi {
    #[oai(path = "/", method = "get")]
    async fn index(&self, #[oai(name="bestilling_id")] bestilling_id: Cookie<Option<i32>>) -> IndexResponse {
        let bestilling_id = bestilling_id.0.unwrap_or(1);
        IndexResponse::Ok(
            PlainText("index billett".to_string()),
            format!("bestilling_id={bestilling_id}")
        )
    }
    #[oai(path = "/token", method = "get")]
    async fn token(
        &self,
        #[oai(name = "my-token")] token: Cookie<Option<String>>,
    ) -> PlainText<String> {
        match token.0 {
            Some(token) => PlainText(token),
            None => PlainText("no token".to_string()),
        }
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
