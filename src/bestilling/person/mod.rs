use crate::ApiTags;

use poem_openapi::{payload::PlainText, OpenApi};
use std::sync::Arc;

pub mod model;
pub mod query;

pub struct PersonApi {
    pub pool: Arc<sqlx::Pool<sqlx::Postgres>>,
}

#[OpenApi(prefix_path = "/v1/person", tag = "ApiTags::Person")]
impl PersonApi {
    #[oai(path = "/", method = "get")]
    async fn index(&self) -> PlainText<String> {
        PlainText("hello persons".to_string())
    }
    async fn ny(&self,) {}
}
