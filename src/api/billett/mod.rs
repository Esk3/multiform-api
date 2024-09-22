use std::{fmt::Debug, sync::Arc};

use poem_openapi::{
    param::{Cookie, Path},
    payload::Json,
    ApiResponse, OpenApi,
};

use crate::ApiTags;

pub mod model;
mod query;

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
    Ok(Json<model::Billett>, #[oai(header = "Set-Cookie")] String),
    #[oai(status = 500)]
    InternalError,
}

#[derive(Debug, ApiResponse)]
enum SetReiseIdResponse {
    #[oai(status = 200)]
    Ok(Json<model::Billett>),
    #[oai(status = 500)]
    Err,
}

#[derive(Debug, ApiResponse)]
enum SetPersonIdResponse {
    #[oai(status = 200)]
    Ok(Json<model::Billett>),
    #[oai(status = 500)]
    Err,
}

#[derive(Debug, ApiResponse)]
enum BekreftBillettResponse {
    #[oai(status = 200)]
    Ok(Json<model::BekreftetBillett>),
    #[oai(status=404)]
    BillettNotFound,
    #[oai(status = 500)]
    Err,
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
    #[oai(path = "/:id", method = "get")]
    async fn get_billett(&self, id: Path<i32>) -> GetBillettResponse {
        let mut tx = self.pool.begin().await.unwrap();
        match query::BillettQuery::new(&mut tx)
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
    async fn create_billett(&self, billett: Json<model::BillettForm>) -> PostBilletResponse {
        let mut tx = self.pool.begin().await.unwrap();
        match query::BillettQuery::new(&mut tx)
            .insert_billet(&billett)
            .await
        {
            Ok(billett) => {
                let billett_id = billett.billett_id;
                tx.commit().await.unwrap();
                PostBilletResponse::Ok(
                    Json(billett),
                    format!("billett_id={billett_id}; SameSite=Lax"),
                )
            }
            Err(e) => {
                dbg!(e);
                PostBilletResponse::InternalError
            }
        }
    }
    #[oai(path = "/reise", method = "put")]
    async fn set_reise_id(
        &self,
        Json(reise_id): Json<i32>,
        Cookie(billett_id): Cookie<i32>,
    ) -> SetReiseIdResponse {
        let mut tx = self.pool.begin().await.unwrap();
        match query::BillettQuery::new(&mut tx)
            .set_reise(billett_id, reise_id)
            .await
        {
            Ok(billett) => {
                tx.commit().await.unwrap();
                SetReiseIdResponse::Ok(Json(billett))
            },
            Err(e) => {
                dbg!(e);
                SetReiseIdResponse::Err
            }
        }
    }
    #[oai(path = "/person", method = "put")]
    async fn set_person_id(
        &self,
        Json(person_id): Json<i32>,
        Cookie(billett_id): Cookie<i32>,
    ) -> SetPersonIdResponse {
        let mut tx = self.pool.begin().await.unwrap();
        match query::BillettQuery::new(&mut tx)
            .set_person(billett_id, person_id)
            .await
        {
            Ok(billett) => {
                tx.commit().await.unwrap();
                SetPersonIdResponse::Ok(Json(billett))
            },
            Err(e) => {
                dbg!(e);
                SetPersonIdResponse::Err
            }
        }
    }
    #[oai(path = "/bekreft", method = "put")]
    async fn bekreft_billett(&self, Cookie(billett_id): Cookie<i32>) -> BekreftBillettResponse {
        let mut tx = self.pool.begin().await.unwrap();
        match query::BillettQuery::new(&mut tx)
            .get_bekreftet_billett_by_id(billett_id)
            .await
        {
            Ok(Some(billett)) => {
                tx.commit().await.unwrap();
                BekreftBillettResponse::Ok(Json(billett))
            },
            Ok(None) => BekreftBillettResponse::Err,
            Err(e) => {
                dbg!(e);
                BekreftBillettResponse::Err
            },
        }
    }
}
