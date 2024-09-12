use std::sync::Arc;

use crate::into_response::IntoResponse;

mod person;
mod billett;

pub struct RouterArgs {
    url: String,
    method: tiny_http::Method,
    pool: Arc<sqlx::PgPool>,
}

impl RouterArgs {
    pub fn new(url: String, method: tiny_http::Method, pool: Arc<sqlx::PgPool>) -> Self {
        Self { url, method, pool }
    }
    pub fn clone_from_http_request(request: &tiny_http::Request, pool: Arc<sqlx::PgPool>) -> Self {
        Self {
            url: request.url().to_string(),
            method: request.method().clone(),
            pool
        }
    }
}

pub struct Router;

impl crate::service::Service<RouterArgs> for Router {
    type Response = Box<dyn IntoResponse>;

    type Error = ();

    type Future = crate::service::Fut<Self::Response, Self::Error>;

    fn call(&mut self, RouterArgs {   url, method, pool }: RouterArgs) -> Self::Future {
        Box::pin(async move {
            let rows = sqlx::query("select * from billett").fetch_all(&*pool).await.unwrap();
            dbg!(rows);
            Ok::<Box<dyn IntoResponse>, ()>(Box::new(()))
        })
    }
}
