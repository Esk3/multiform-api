use poem::{http::HeaderMap, middleware::SetHeader, EndpointExt, IntoResponse};
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, sync::Arc};

use poem::Endpoint;
use poem_openapi::{param::{Cookie, Header, Path}, payload::PlainText, ApiRequest, ApiResponse, OpenApi, SecurityScheme};

mod get_billett;
mod lagre_billett;
pub mod model;

#[derive(ApiResponse)]
enum IndexResponse {
    #[oai(status=200)]
    Ok(PlainText<String>, #[oai(header="Set-Cookie")] String)
}

pub struct BilletApi {
    pub pool: Arc<sqlx::Pool<sqlx::Postgres>>,
}

#[OpenApi(prefix_path="/v1/billett")]
impl BilletApi {
    #[oai(path = "/", method = "get")]
    async fn index(&self, headers: &HeaderMap) -> IndexResponse {
        let billett_id = headers.get("cookie").map(|cookie| {
            dbg!(cookie
                .to_str()
                .unwrap()
                .split_once('=')
                .unwrap()
                .1
                .parse::<i32>()
                .unwrap())
        });
        dbg!(billett_id);
        IndexResponse::Ok(PlainText("index billett".to_string()), "billett_id=1".to_string())
    }
    #[oai(path="/token", method="get")]
    async fn token(&self, #[oai(name="my-token")] token: Cookie<String>) -> PlainText<String> {
        PlainText(token.to_string())
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
