use std::sync::Arc;

use poem_openapi::{payload::PlainText, OpenApi};

use crate::{
    error::ServerError,
    into_response::IntoResponse,
    service::{Fut, Service},
};

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
}

pub fn handler() -> impl Service<Args, Response = Box<dyn IntoResponse>, Error = ServerError> + Clone
{
    Handler
}

#[derive(Clone)]
struct Handler;
impl Service<Args> for Handler {
    type Response = Box<dyn IntoResponse>;

    type Error = ServerError;

    type Future = Fut<Self::Response, Self::Error>;

    fn call(&mut self, request: Args) -> Self::Future {
        Box::pin(async move {
            match request.method {
                tiny_http::Method::Get => get_billett(request.bestillings_id, request.pool).await,
                _ => todo!(),
            }
        })
    }
}

async fn get_billett(
    id: i32,
    pool: Arc<sqlx::PgPool>,
) -> Result<Box<dyn IntoResponse>, ServerError> {
    dbg!("quering billett med id ", id);
    let result: model::Billett = dbg!(dbg!(
        sqlx::query_as("select bestillings_id, fra_iata_code, til_iata_code, status::text, billett_type::text, timestamp::text from billett where bestillings_id = $1")
            .bind(id)
            .fetch_one(&*pool)
            .await
    )
    .map_err(|_| ())?);
    dbg!(&result);
    Ok(Box::new(format!(
        "
        fra: {}
        til: {}
        status: {}
        billett type: {}
            ",
        result.fra_iata_code, result.til_iata_code, result.status, result.billett_type
    )))
}
