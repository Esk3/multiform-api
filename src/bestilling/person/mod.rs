use std::sync::Arc;

use poem_openapi::{payload::PlainText, OpenApi};

pub mod model;

pub struct PersonApi {
    pub pool: Arc<sqlx::Pool<sqlx::Postgres>>,
}

#[OpenApi]
impl PersonApi {
    #[oai(path = "/person", method = "get")]
    async fn index(&self) -> PlainText<String> {
        PlainText("hello persons".to_string())
    }
}
