use crate::ApiTags;

use poem_openapi::{payload::Json, ApiResponse, OpenApi};
use std::sync::Arc;

pub mod model;
pub mod query;

#[derive(ApiResponse)]
enum CreatePersonResponse {
    #[oai(status = 201)]
    Ok(Json<model::Person>),
    #[oai(status = 500)]
    Err,
}

pub struct PersonApi {
    pool: Arc<sqlx::Pool<sqlx::Postgres>>,
}

impl PersonApi {
    pub fn new(pool: Arc<sqlx::Pool<sqlx::Postgres>>) -> Self {
        Self { pool }
    }
}

#[OpenApi(prefix_path = "/v1/person", tag = "ApiTags::Person")]
impl PersonApi {
    #[oai(path = "/", method = "post")]
    async fn create_person(&self, Json(person): Json<model::PersonForm>) -> CreatePersonResponse {
        match query::PersonQuery::new(self.pool.clone())
            .insert_person(&person)
            .await
        {
            Ok(person) => CreatePersonResponse::Ok(Json(person)),
            Err(e) => {
                dbg!(e);
                CreatePersonResponse::Err
            }
        }
    }
}
