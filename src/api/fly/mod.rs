use crate::ApiTags;

use poem_openapi::{param::Path, payload::Json, ApiResponse, OpenApi};
use query::FlyQuery;
use std::sync::Arc;

pub mod model;
mod query;

#[derive(ApiResponse)]
enum FlyListResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<model::Fly>>),
    #[oai(status = 500)]
    Err,
}

#[derive(ApiResponse)]
enum FlyResponse {
    #[oai(status = 200)]
    Ok(Json<model::Fly>),
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    Err,
}

#[derive(ApiResponse)]
enum CreateFlyResponse {
    #[oai(status = 201)]
    Ok(Json<model::Fly>),
    #[oai(status = 500)]
    Err,
}

pub struct FlyApi {
    pool: Arc<sqlx::Pool<sqlx::Postgres>>,
}

impl FlyApi {
    pub fn new(pool: Arc<sqlx::Pool<sqlx::Postgres>>) -> Self {
        Self { pool }
    }
}

#[OpenApi(prefix_path = "/v1/fly", tag = "ApiTags::Fly")]
impl FlyApi {
    #[oai(path = "/", method = "get")]
    async fn fly_liste(&self) -> FlyListResponse {
        let mut tx = self.pool.begin().await.unwrap();
        match FlyQuery::new(&mut tx).get_fly().await {
            Ok(fly) => FlyListResponse::Ok(Json(fly)),
            Err(e) => {
                dbg!(e);
                FlyListResponse::Err
            }
        }
    }

    #[oai(path = "/:id", method = "get")]
    async fn fly_by_id(&self, Path(id): Path<i32>) -> FlyResponse {
        let mut tx = self.pool.begin().await.unwrap();
        match FlyQuery::new(&mut tx).get_fly_by_id(id).await {
            Ok(Some(fly)) => FlyResponse::Ok(Json(fly)),
            Ok(None) => FlyResponse::NotFound,
            Err(e) => {
                dbg!(e);
                FlyResponse::Err
            }
        }
    }

    #[oai(path = "/", method = "post")]
    async fn create_fly(&self, Json(fly_form): Json<model::FlyForm>) -> CreateFlyResponse {
        let mut tx = self.pool.begin().await.unwrap();
        match FlyQuery::new(&mut tx).create_fly(fly_form).await {
            Ok(fly) => {
                tx.commit().await.unwrap();
                CreateFlyResponse::Ok(Json(fly))
            }
            Err(e) => {
                dbg!(e);
                CreateFlyResponse::Err
            }
        }
    }
}
