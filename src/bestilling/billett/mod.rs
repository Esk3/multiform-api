use std::{fmt::Debug, sync::Arc};

use poem_openapi::{
    param::{Cookie, Path},
    payload::{Json, PlainText},
    ApiResponse, OpenApi,
};

mod get_billett;
mod lagre_billett;
pub mod model;
mod query;

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
    async fn index(
        &self,
        #[oai(name = "bestilling_id")] bestilling_id: Cookie<Option<i32>>,
    ) -> IndexResponse {
        let bestilling_id = bestilling_id.0.unwrap_or(1);
        IndexResponse::Ok(
            PlainText("index billett".to_string()),
            format!("bestilling_id={bestilling_id}"),
        )
    }
    #[oai(path = "/:id", method = "get")]
    async fn get_billett(&self, id: Path<i32>) -> GetBillettResponse {
        match query::BillettQuery::new(self.pool.clone())
            .get_billett_by_id(*id)
            .await
        {
            Ok(Some(billett)) => GetBillettResponse::Ok(Json(billett)),
            Ok(None) => GetBillettResponse::NotFound,
            Err(e) => {
                dbg!(e);
                GetBillettResponse::InternalError
            },
        }
    }
}

#[derive(Debug, ApiResponse)]
enum GetBillettResponse {
    #[oai(status = 200)]
    Ok(Json<model::Billett>),
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    InternalError,
}
