use std::{fmt::Debug, sync::Arc};

use poem_openapi::{
    param::{Cookie, Path},
    payload::{Json, PlainText},
    ApiResponse, OpenApi,
};

use super::{bestilling_id, model::Bestilling};

mod get_billett;
mod lagre_billett;
pub mod model;
mod query;

#[derive(ApiResponse)]
enum IndexResponse {
    #[oai(status = 200)]
    Ok(PlainText<String>, #[oai(header = "Set-Cookie")] String),
    #[oai(status = 500)]
    Error,
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

#[derive(Debug, ApiResponse)]
enum PostBilletResponse {
    #[oai(status = 201)]
    Ok,
    #[oai(status = 500)]
    InternalError,
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
        let Ok(bestilling_id) = BestillingsId::new(*bestilling_id)
            .get_or_create(self.pool.clone())
            .await
        else {
            return IndexResponse::Error;
        };
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
            }
        }
    }
    #[oai(path = "/", method = "post")]
    async fn post_billett(
        &self,
        #[oai(name = "bestillings_id")] bestilling_id: Cookie<Option<i32>>,
    ) -> PostBilletResponse {
        let Ok(bestilling_id) = BestillingsId::new(*bestilling_id)
            .get_or_create(self.pool.clone())
            .await
        else {
            return PostBilletResponse::InternalError;
        };
        match query::BillettQuery::new(self.pool.clone())
            .insert_billet()
            .await
        {
            Ok(_) => PostBilletResponse::Ok,
            Err(_) => PostBilletResponse::InternalError,
        }
    }
}

struct BestillingsId(Option<i32>);
impl BestillingsId {
    fn new(id: Option<i32>) -> Self {
        Self(id)
    }
    async fn get_or_create(
        &self,
        _pool: Arc<sqlx::Pool<sqlx::Postgres>>,
    ) -> Result<i32, sqlx::Error> {
        dbg!("todo query db for id");
        Ok(self.0.unwrap_or(1))
    }
}
