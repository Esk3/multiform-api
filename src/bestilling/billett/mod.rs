use std::{fmt::Debug, sync::Arc};

use poem_openapi::{
    param::{Cookie, Path},
    payload::{Json, PlainText},
    ApiResponse, OpenApi,
};

use crate::{ApiTags, BestillingsId};

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
    Ok(#[oai(header = "Set-Cookie")] String),
    #[oai(status = 500)]
    InternalError,
}

pub struct BilletApi {
    pool: Arc<sqlx::Pool<sqlx::Postgres>>,
}

impl BilletApi {
    pub fn new(pool: Arc<sqlx::Pool<sqlx::Postgres>>) -> Self {
        Self { pool }
    }
}

#[OpenApi(prefix_path = "/v1/billett", tag = "ApiTags::Billett")]
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
        billett: Json<model::BillettForm>,
        #[oai(name = "bestilling_id")] id: Cookie<Option<i32>>,
    ) -> PostBilletResponse {
        let id = BestillingsId::new(*id)
            .get_or_create(self.pool.clone())
            .await
            .unwrap();
        match query::BillettQuery::new(self.pool.clone())
            .insert_billet(&billett, id)
            .await
        {
            Ok(row) => {
                dbg!(row);
                PostBilletResponse::Ok(format!("bestilling_id{id}"))
            }
            Err(e) => {
                dbg!(e);
                PostBilletResponse::InternalError
            }
        }
    }
}
